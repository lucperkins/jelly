use clap::{Command, Parser};

#[derive(Parser)]
#[command(version, about = "An all-in-one documentation generation tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Build(Build),
    Index(Index),
}
