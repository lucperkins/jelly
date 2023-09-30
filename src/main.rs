use clap::{Args, Parser, Subcommand};
use jelly::cmd::{build, index};
use std::{io::IsTerminal, path::PathBuf, process::ExitCode};

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
#[command(about = "Generate a search index for a Jelly docs project")]
struct Index {
    #[arg(
        short,
        long,
        help = "The root content directory",
        default_value = "docs"
    )]
    source: PathBuf,

    #[arg(short, long = "out", help = "Output path")]
    out: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Command {
    Build(Build),
    Index(Index),
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

fn main() -> color_eyre::Result<ExitCode> {
    color_eyre::config::HookBuilder::default()
        .theme(if !std::io::stderr().is_terminal() {
            color_eyre::config::Theme::new()
        } else {
            color_eyre::config::Theme::dark()
        })
        .install()?;

    let cli = Cli::parse();

    match cli.command {
        Command::Build(Build { source, out }) => build(source, out),
        Command::Index(Index { source, out }) => index(source, out),
    }
}
