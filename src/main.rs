use clap::{Args, Parser, Subcommand};
use jelly::{config::Config, error::ContentError, page::TitleConfig, section::get_sections};
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

            let sections = get_sections(&config)?;

            for section in sections {
                println!(
                    "(title: {title}, num_pages: {num})",
                    title = section.title,
                    num = section.pages.len()
                );
            }
        }
    }

    Ok(())
}
