use clap::Parser;
use camino::Utf8PathBuf;
use anyhow::Result;
use super::execute::*;
use crate::command::Config;
use frame_metadata::RuntimeMetadata;

/// Decode hex data with a specific runtime metadata type.
#[derive(Debug, Clone, Parser)]
pub struct DecodeCmd {
	/// The runtime metadata type to decode the data as.
	#[clap(short = 't', long = "type")]
	pub as_typ: String,

	/// The hex data to decode.
	#[clap(short, long)]
	pub data: String,
}

impl DecodeCmd {
	pub fn run(&self, cfg: &Config) -> Result<()> {
		let data = hex::decode(&self.data)?;
		let metadata = get_metadata(&cfg.runtime)?;

		

		Ok(())
	}
}

fn get_metadata(runtime: &Utf8PathBuf) -> Result<RuntimeMetadata> {
	let raw_meta = call_api(runtime, METADATA, metadata::METADATA)?;
	let meta = decode_metadata(raw_meta)?;
	Ok(meta.1)
}
