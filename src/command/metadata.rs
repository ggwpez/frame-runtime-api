use camino::Utf8PathBuf;
use clap::{Parser, Subcommand, ValueEnum};
use anyhow::Result;
use super::execute::*;
use crate::command::Config;
use frame_metadata::RuntimeMetadata;
use std::collections::BTreeSet;
use scale_info::{PortableRegistry, PortableType};

/// List metadata information.
#[derive(Debug, Clone, Parser)]
pub struct MetadataCmd {
	#[clap(subcommand)]
	sub: MetadataSub,
}

#[derive(Debug, Clone, Subcommand)]
pub enum MetadataSub {
	/// List somthing in the metadata.
	List(ListCmd),
	/// Find something in the metadata.
	Find(FindCmd),
}

/// List something.
#[derive(Debug, Clone, Parser)]
pub struct ListCmd {
	/// What to list.
	#[clap(value_enum, index = 1)]
	pub what: What,

	/// Skip empty types.
	#[clap(long, default_value = "true")]
	pub skip_empty: Option<bool>,
}

/// List something.
#[derive(Debug, Clone, Parser)]
pub struct FindCmd {
	/// What to find.
	#[clap(value_enum, index = 1)]
	pub what: What,

	/// The runtime metadata type to find.
	#[clap(index = 2)]
	pub value: String,
}

#[derive(Debug, Clone, PartialEq, clap::ValueEnum)]
pub enum What {
	/// List all types in the metadata.
	#[clap(alias = "type")]
	Types,
}

impl MetadataCmd {
	pub fn run(&self, cfg: &Config) -> Result<()> {
		match &self.sub {
			MetadataSub::List(cmd) => cmd.run(cfg),
			MetadataSub::Find(cmd) => cmd.run(cfg),
		}
	}
}

impl ListCmd {
	pub fn run(&self, cfg: &Config) -> Result<()> {
		match self.what {
			What::Types => (), // static assert
		};
		let reg = extract_registry(&cfg.runtime)?;

		let mut found = BTreeSet::new();
		for t in reg.types.iter() {
			let s = t.ty.path.to_string();
			if self.skip_empty.unwrap_or(true) && s.is_empty() {
				continue;
			}

			found.insert(s);
		}

		for t in found.iter() {
			println!("{}", t);
		}

		Ok(())
	}
}

impl FindCmd {
	pub fn run(&self, cfg: &Config) -> Result<()> {
		match self.what {
			What::Types => (), // static assert
		};
		let reg = extract_registry(&cfg.runtime)?;

		let mut found = BTreeSet::new();
		for t in reg.types.iter() {
			let s = t.ty.path.to_string();
			if s.contains(&self.value) {
				found.insert(s);
			}
		}

		if found.is_empty() {
			return Err(anyhow::anyhow!("Type not found in metadata"));
		}

		for s in found.iter() {
			println!("{s}");
		}

		if found.len() > 1 {
			return Err(anyhow::anyhow!("Multiple types found in metadata"));
		}
		
		Ok(())
	}
}

fn extract_registry(runtime: &Utf8PathBuf) -> Result<PortableRegistry> {
	let metadata = get_metadata(runtime)?;

	match metadata {
		RuntimeMetadata::V14(v) => Ok(v.types),
		RuntimeMetadata::V15(v) => Ok(v.types),
		_ => Err(anyhow::anyhow!("Unsupported metadata version")),
	}
}
