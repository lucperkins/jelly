use clap::{Args, Parser, Subcommand};
use jelly::{
    config::{SiteConfig, TitleConfig},
    error::ContentError,
    site::build_site,
};
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

            let config = SiteConfig {
                root,
                title_config: TitleConfig::default(),
            };

            let site = build_site(&config)?;
            let site_as_str = serde_json::to_string_pretty(&site)?;
            println!("{}", site_as_str);
        }
    }

    Ok(())
}
