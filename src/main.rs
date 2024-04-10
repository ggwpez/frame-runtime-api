pub mod command;

use anyhow::Result;
use clap::Parser;
use command::Command;

fn main() -> Result<()> {
    Command::parse().run()
}
