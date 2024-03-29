use clap::{Args, Parser, Subcommand};
use jelly::cmd::{build, index};
use std::{io::IsTerminal, path::PathBuf, process::ExitCode};

/// Build a Jelly docs project
#[derive(Args)]
struct Build {
    /// The root content directory
    #[arg(short, long, default_value = "docs")]
    source: PathBuf,

    /// Output directory
    #[arg(short, long, default_value = "dist")]
    out: PathBuf,
}

/// Generate a search index for a Jelly docs project
#[derive(Args)]
struct Index {
    /// The root content directory
    #[arg(short, long, default_value = "docs")]
    source: PathBuf,

    /// Output path
    #[arg(short, long = "out")]
    out: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Command {
    Build(Build),
    Index(Index),
}

/// Jelly: golden path static site generator for documentation
#[derive(Parser)]
#[command(version)]
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
