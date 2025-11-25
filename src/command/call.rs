use crate::command::Config;
use anyhow::Result;
use camino::Utf8PathBuf;
use clap::Parser;
use frame_metadata::RuntimeMetadata;
use frame_metadata::RuntimeMetadataPrefixed;
use sc_executor::WasmExecutor;
use scale::Decode;
use sp_core::traits::CallContext;
use sp_core::traits::CodeExecutor;
use sp_core::traits::RuntimeCode;
use sp_core::traits::WrappedRuntimeCode;
use sp_state_machine::BasicExternalities;
use std::borrow::Cow;
use std::fmt::Debug;

/// Call a runtime API.
#[derive(Debug, Clone, Parser)]
pub struct CallCmd {
    #[clap(index = 1, required = true)]
    pub api: String,

    #[clap(index = 2, required = true)]
    pub call: String,

    /// Print output as hex.
    #[clap(long)]
    pub hex: bool,

    #[clap(long)]
    pub arg_file: Option<Utf8PathBuf>,

    #[clap(long, conflicts_with = "arg_file")]
    pub arg: Option<String>,
}

pub const METADATA: &str = "Metadata";
pub mod metadata {
    pub const VERSIONS: &str = "metadata_versions";
    pub const METADATA: &str = "metadata";
}

pub const CORE: &str = "Core";
pub mod core {
    pub const VERSION: &str = "version";
}

impl CallCmd {
    pub fn run(&self, cfg: &Config) -> Result<()> {
        sp_tracing::try_init_simple();
        let arg = match (&self.arg_file, &self.arg) {
            (Some(file), _) => {
                let content = String::from_utf8(std::fs::read(file)?)?;
                hex::decode(content.trim_start_matches("0x").trim())?
            }
            (_, Some(arg)) => hex::decode(arg.trim_start_matches("0x").trim())?,
            _ => vec![],
        };
        let runtime = cfg.get_runtime()?;
        let res = call_api(&runtime, &self.api, &self.call, arg)?;

        if self.hex {
            println!("0x{}", hex::encode(res));
            return Ok(());
        }

        match (self.api.as_str(), self.call.as_str()) {
            (METADATA, metadata::VERSIONS) => print_result::<Vec<u32>>(res),
            (METADATA, metadata::METADATA) => print_metadata(res),
            (CORE, core::VERSION) => print_result::<sc_executor::RuntimeVersion>(res),
            _ => print_best_effort(res),
        }
    }
}

pub fn call_api(runtime: &Utf8PathBuf, api: &str, call: &str, arg: Vec<u8>) -> Result<Vec<u8>> {
    let (code, hash) = extract_wasm(&runtime)?;
    let mut ext = BasicExternalities::new_empty();

    let exe = WasmExecutor::<sp_io::SubstrateHostFunctions>::builder()
        .with_allow_missing_host_functions(true)
        .build();

    let method = format!("{}_{}", api, call);
    let (res, used_native) = exe.call(
        &mut ext,
        &RuntimeCode {
            heap_pages: None,
            code_fetcher: &code,
            hash,
        },
        &method,
        &arg,
        CallContext::Offchain,
    );
    assert!(!used_native);
    res.map_err(Into::into)
}

fn print_result<T: Decode + Debug>(data: Vec<u8>) -> Result<()> {
    let data = T::decode(&mut &data[..])?;
    println!("{:#?}", data);
    Ok(())
}

pub fn decode_metadata(data: Vec<u8>) -> Result<RuntimeMetadataPrefixed> {
    let meta = frame_metadata::OpaqueMetadata::decode(&mut &data[..])?.0;

    if let Ok(v) = RuntimeMetadataPrefixed::decode(&mut &meta[..]) {
        return Ok(v);
    }

    anyhow::bail!("Could not decode metadata as RuntimeMetadataPrefixed");
}

fn print_metadata(data: Vec<u8>) -> Result<()> {
    if let Ok(v) = decode_metadata(data) {
        println!("{:#?}", v);
        return Ok(());
    }

    anyhow::bail!("Could not decode metadata as RuntimeMetadataPrefixed");
}

fn print_best_effort(data: Vec<u8>) -> Result<()> {
    if print_result_non_string(data.clone()).is_ok() {
        return Ok(());
    }

    if !data.contains(&0) {
        let Ok(data) = std::str::from_utf8(&mut &data[..]) else {
            println!("{}", hex::encode(data));
            return Ok(());
        };

        if let Ok(data) = serde_json::from_str::<serde_json::Value>(&data) {
            println!("{}", serde_json::to_string_pretty(&data)?);
            return Ok(());
        };
    }

    println!("0x{}", hex::encode(data));
    Ok(())
}

fn print_result_non_string(data: Vec<u8>) -> Result<()> {
    if let Ok(data) = Vec::<String>::decode(&mut &data[..]) {
        println!("{:#?}", data);
        return Ok(());
    }

    anyhow::bail!("Could not decode result as JSON or Vec<&str>");
}

fn extract_wasm(runtime: &Utf8PathBuf) -> Result<(WrappedRuntimeCode<'static>, Vec<u8>)> {
    log::info!("Loading WASM from {}", runtime);
    let code = std::fs::read(runtime)?;
    let hash = sp_crypto_hashing::blake2_256(&code).to_vec();
    let wrapped_code = WrappedRuntimeCode(Cow::Owned(code));

    Ok((wrapped_code, hash))
}

pub fn get_metadata(runtime: &Utf8PathBuf) -> Result<RuntimeMetadata> {
    let raw_meta = call_api(runtime, METADATA, metadata::METADATA, vec![])?;
    let meta = decode_metadata(raw_meta)?;
    Ok(meta.1)
}
