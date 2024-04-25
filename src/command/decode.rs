use crate::command::{call::*, Config};
use anyhow::Result;
use camino::Utf8PathBuf;
use clap::Parser;
use frame_metadata::RuntimeMetadata;
use scale_decode::DecodeAsType;
use scale_value::Value;

/// Decode data with a specific runtime metadata type.
#[derive(Debug, Clone, Parser)]
pub struct DecodeCmd {
    /// The runtime metadata type to decode the data as.
    #[clap(short = 't', long = "type", index = 1)]
    pub as_typ: String,

    /// The hex data to decode.
    #[clap(short, long)]
    pub data: String,
}

impl DecodeCmd {
    pub fn run(&self, cfg: &Config) -> Result<()> {
        self.check_args()?;
        let data = hex::decode(&self.data.trim_start_matches("0x"))?;
        let metadata = get_metadata(&cfg.runtime)?;

        let registry = match metadata {
            RuntimeMetadata::V14(v) => v.types,
            RuntimeMetadata::V15(v) => v.types,
            _ => return Err(anyhow::anyhow!("Unsupported metadata version")),
        };

        let t = registry
            .types
            .iter()
            .find(|t| t.ty.path.to_string().ends_with(&self.as_typ))
            .ok_or_else(|| anyhow::anyhow!("Type not found in metadata"))?;

        let v = Value::decode_as_type(&mut &data[..], &t.id, &registry)?;
        let json = serde_json::to_string_pretty(&v)?;
        println!("{}", json);

        Ok(())
    }

    fn check_args(&self) -> Result<()> {
        if self.as_typ.is_empty() {
            return Err(anyhow::anyhow!("Type is required"));
        }

        Ok(())
    }
}

fn get_metadata(runtime: &Utf8PathBuf) -> Result<RuntimeMetadata> {
    let raw_meta = call_api(runtime, METADATA, metadata::METADATA, vec![])?;
    let meta = decode_metadata(raw_meta)?;
    Ok(meta.1)
}
