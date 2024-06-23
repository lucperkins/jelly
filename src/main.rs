use clap::Parser;
use jelly::Cli;

fn main() -> color_eyre::Result<()> {
    Cli::parse().execute()
}
