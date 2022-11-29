use clap::{Args, Parser, Subcommand};
use jelly::{build_site, Config, ContentError, TitleConfig};
use std::path::PathBuf;

#[derive(Args)]
#[command(about = "Build a Jelly project")]
struct Build {
    #[arg(short, long, default_value = ".")]
    source: PathBuf,
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
        Build(args) => {
            let root = args.source;

            let config = Config {
                root,
                title_config: TitleConfig::default(),
            };

            build_site(&config)?;
        }
    }

    Ok(())
}
