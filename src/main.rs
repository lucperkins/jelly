use clap::{Args, Parser, Subcommand};
use jelly::{error::ContentError, site::build_site};
use std::path::PathBuf;

#[derive(Args)]
#[command(about = "Build a Jelly docs project")]
struct Build {
    #[arg(
        short,
        long,
        help = "The root content directory",
        default_value = "./docs"
    )]
    source: PathBuf,

    #[arg(
        short,
        long = "out",
        help = "Output directory",
        default_value = "./dist"
    )]
    out: PathBuf,
}

#[derive(Subcommand)]
enum Command {
    Build(Build),
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

fn main() -> Result<(), ContentError> {
    use Command::*;

    let cli = Cli::parse();

    match cli.command {
        Build(args) => build_site(args.source, args.out),
    }
}
