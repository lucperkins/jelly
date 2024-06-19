use clap::{Parser, Subcommand};

use jelly::{build, index, serve, JellyError};
use std::io::IsTerminal;
use std::path::PathBuf;
use tracing_subscriber::EnvFilter;

/// Build a Jelly docs project
#[derive(Parser)]
#[command(alias = "b", alias = "bld", alias = "bu")]
struct Build {
    /// The root content directory
    #[arg(short, long, default_value = "./docs")]
    source: PathBuf,

    /// Output directory
    #[arg(short, long, default_value = "./dist")]
    out: PathBuf,
}

/// Serve a Jelly docs project
#[derive(Parser)]
#[command(alias = "s", alias = "se", alias = "sr", alias = "srv")]
struct Serve {
    #[arg(
        short,
        long,
        help = "The root content directory",
        default_value = "./docs"
    )]
    source: PathBuf,

    #[arg(short, long, help = "Open the browser to the running site")]
    open: bool,

    #[arg(
        short,
        long,
        default_value_t = 3000,
        help = "The HTTP port to listen on"
    )]
    port: u16,
}

/// Generate a search index for a Jelly docs project
#[derive(Parser)]
#[command(alias = "i", alias = "idx")]
struct Index {
    /// The root content directory
    #[arg(short, long, default_value = "./docs")]
    source: PathBuf,

    /// Output path
    #[arg(short, long = "out")]
    out: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Command {
    Build(Build),
    Index(Index),
    Serve(Serve),
}

/// Jelly: golden path static site generator for documentation
#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

fn main() -> Result<(), JellyError> {
    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    color_eyre::config::HookBuilder::default()
        .theme(if !std::io::stderr().is_terminal() {
            color_eyre::config::Theme::new()
        } else {
            color_eyre::config::Theme::dark()
        })
        .install()
        .expect("couldn't initialize eyre");

    let Cli { command } = Cli::parse();

    match command {
        Command::Build(Build { source, out }) => build(&source, &out),
        Command::Index(Index { source, out }) => index(source, out),
        Command::Serve(Serve { source, open, port }) => serve(source, open, port),
    }
}
