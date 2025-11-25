mod call;
mod decode;
mod metadata;

use anyhow::{anyhow, bail, Result};
use camino::Utf8PathBuf;
use clap::Parser;
use clap::Subcommand;

#[derive(Debug, Parser)]
pub struct Command {
    #[clap(subcommand)]
    sub: Sub,

    #[clap(flatten)]
    config: Config,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Sub {
    Call(call::CallCmd),
    Decode(decode::DecodeCmd),
    Metadata(metadata::MetadataCmd),
}

#[derive(Debug, Parser)]
pub struct Config {
    #[clap(short, long)]
    pub runtime: Option<Utf8PathBuf>,
}

impl Config {
    /// Get the runtime path, either from the explicit argument or by auto-detecting
    /// a single .wasm file in the current directory.
    pub fn get_runtime(&self) -> Result<Utf8PathBuf> {
        if let Some(runtime) = &self.runtime {
            return Ok(runtime.clone());
        }

        // Try to find a .wasm file in the current directory
        let current_dir = std::env::current_dir()
            .map_err(|e| anyhow!("Failed to get current directory: {}", e))?;

        let mut wasm_files = Vec::new();
        if let Ok(entries) = std::fs::read_dir(&current_dir) {
            for entry in entries.flatten() {
                if let Ok(file_name) = entry.file_name().into_string() {
                    if file_name.contains(".wasm") {
                        wasm_files.push(file_name);
                    }
                }
            }
        }

        match wasm_files.len() {
            0 => bail!("No .wasm file found in current directory. Please specify one with -r/--runtime"),
            1 => {
                Ok(Utf8PathBuf::from(&wasm_files[0]))
            },
            _ => {
                bail!("Multiple .wasm files found in current directory: {:?}. Please specify one with -r/--runtime", wasm_files)
            }
        }
    }
}

impl Command {
    pub fn run(&self) -> Result<()> {
        match &self.sub {
            Sub::Call(cmd) => cmd.run(&self.config),
            Sub::Decode(cmd) => cmd.run(&self.config),
            Sub::Metadata(cmd) => cmd.run(&self.config),
        }
    }
}
