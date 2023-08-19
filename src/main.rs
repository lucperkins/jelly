use clap::{Args, Parser, Subcommand};

use jelly::{
    cmd::{build, serve},
    error::Error,
};
use std::path::PathBuf;

#[derive(Args)]
#[command(about = "Build a Jelly docs project")]
struct Build {
    #[arg(
        short,
        long,
        help = "The root content directory",
        default_value = "docs"
    )]
    source: PathBuf,

    #[arg(short, long = "out", help = "Output directory", default_value = "dist")]
    out: PathBuf,
}

#[derive(Args)]
#[command(about = "Serve a Jelly docs project")]
struct Serve {
    #[arg(
        short,
        long,
        help = "The root content directory",
        default_value = "docs"
    )]
    source: PathBuf,
}

#[derive(Subcommand)]
enum Command {
    Build(Build),
    Serve(Serve),
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

fn main() -> Result<(), Error> {
    let Cli { command } = Cli::parse();

    match command {
        Command::Build(Build { source, out }) => build(source, out),
        Command::Serve(Serve { source }) => serve(source),
    }
}
