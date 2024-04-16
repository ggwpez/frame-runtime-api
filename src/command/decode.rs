use clap::Parser;
use camino::Utf8PathBuf;
use anyhow::Result;
use super::execute::*;
use crate::command::Config;
use frame_metadata::RuntimeMetadata;
use std::collections::BTreeSet;

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

		let t = registry.types.iter().find(|t| t.ty.path.to_string().ends_with(&self.as_typ))
			.ok_or_else(|| anyhow::anyhow!("Type not found in metadata"))?;
		println!("{:?}", t);

		t.ty.into_portable();

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
	let raw_meta = call_api(runtime, METADATA, metadata::METADATA)?;
	let meta = decode_metadata(raw_meta)?;
	Ok(meta.1)
}
