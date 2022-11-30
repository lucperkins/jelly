use clap::{Args, Parser, Subcommand};
use jelly::{config::Config, error::ContentError, page::TitleConfig, site::build_site};
use std::path::PathBuf;

#[derive(Args)]
#[command(about = "Build a Jelly project")]
struct Build {
    #[arg(short, long, help = "The root docs directory", default_value = "docs")]
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

            let site = build_site(&config)?;

            for section in site.sections {
                println!("{:?}", section);
            }
        }
    }

    Ok(())
}
