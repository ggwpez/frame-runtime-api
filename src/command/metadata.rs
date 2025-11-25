use crate::command::{call::*, Config};
use anyhow::Result;
use camino::Utf8PathBuf;
use clap::{Parser, Subcommand};
use frame_metadata::RuntimeMetadata;
use regex::Regex;
use scale_info::PortableRegistry;
use serde_json;
use std::collections::BTreeSet;
use std::fs;

/// List metadata information.
#[derive(Debug, Clone, Parser)]
pub struct MetadataCmd {
    #[clap(subcommand)]
    sub: MetadataSub,
}

#[derive(Debug, Clone, Subcommand)]
pub enum MetadataSub {
    /// Show types in the metadata.
    Show(ShowCmd),
    /// Write metadata to JSON file.
    WriteJson(WriteJsonCmd),
}

/// Show types in the metadata.
#[derive(Debug, Clone, Parser)]
pub struct ShowCmd {
    /// What to show.
    #[clap(value_enum, index = 1)]
    pub what: What,

    /// Optional regex pattern to filter types (case-sensitive).
    #[clap(index = 2)]
    pub pattern: Option<String>,

    /// Show detailed information about the types.
    #[clap(short, long)]
    pub details: bool,
}

/// Write metadata to JSON file.
#[derive(Debug, Clone, Parser)]
pub struct WriteJsonCmd {
    /// Output JSON file path.
    #[clap(index = 1)]
    pub output: Utf8PathBuf,
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
            MetadataSub::Show(cmd) => cmd.run(cfg),
            MetadataSub::WriteJson(cmd) => cmd.run(cfg),
        }
    }
}

impl ShowCmd {
    pub fn run(&self, cfg: &Config) -> Result<()> {
        match self.what {
            What::Types => (), // static assert
        };
        let reg = extract_registry(&cfg.runtime)?;

        // Filter types by pattern, if provided
        let matching_types: Vec<_> = if let Some(pattern) = &self.pattern {
            let regex = Regex::new(pattern)?;
            reg.types
                .iter()
                .filter(|t| regex.is_match(&t.ty.path.to_string()))
                .collect()
        } else {
            reg.types.iter().collect()
        };

        if matching_types.is_empty() {
            return Err(anyhow::anyhow!("No types found matching the pattern"));
        }

        // Collect unique type names and sort
        let mut type_names = BTreeSet::new();
        for t in &matching_types {
            type_names.insert(t.ty.path.to_string());
        }

        // Print type names
        for name in &type_names {
            println!("{}", name);

            if self.details {
                // Find and print details for all types with this name
                let mut types_with_name: Vec<_> = matching_types
                    .iter()
                    .filter(|t| t.ty.path.to_string() == *name)
                    .collect();
                types_with_name.sort_by_key(|t| t.id);

                for t in types_with_name {
                    println!("{:#?}", t.ty.type_def);
                }
            }
        }

        Ok(())
    }
}

impl WriteJsonCmd {
    pub fn run(&self, cfg: &Config) -> Result<()> {
        let reg = extract_registry(&cfg.runtime)?;

        let json = serde_json::to_string_pretty(&reg)?;
        fs::write(&self.output, json)?;

        println!("Metadata written to {}", self.output);
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
