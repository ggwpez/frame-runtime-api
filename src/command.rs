mod decode;
mod call;
mod metadata;

use anyhow::Result;
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
    pub runtime: Utf8PathBuf,
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
