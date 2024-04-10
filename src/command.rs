mod execute;
mod decode;

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
    Execute(execute::ExecuteCmd),
    Decode(decode::DecodeCmd),
}

#[derive(Debug, Parser)]
pub struct Config {
    #[clap(short, long)]
    pub runtime: Utf8PathBuf,
}

impl Command {
    pub fn run(&self) -> Result<()> {
        match &self.sub {
            Sub::Execute(cmd) => cmd.run(&self.config),
            Sub::Decode(cmd) => cmd.run(&self.config),
        }
    }
}
