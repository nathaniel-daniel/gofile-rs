mod commands;
mod config;
mod util;

pub use self::config::Config;
use anyhow::Context;
use clap::Parser;
use etcetera::AppStrategy;
use etcetera::AppStrategyArgs;
use shadow_rs::shadow;
use std::path::PathBuf;

shadow!(build);

pub fn get_config_dir() -> anyhow::Result<PathBuf> {
    let app_strategy = etcetera::choose_app_strategy(AppStrategyArgs {
        app_name: "gofile-cli".into(),
        author: "".into(),
        top_level_domain: "".into(),
    })?;

    let config_dir = app_strategy.config_dir();

    // Create config dir if it does not exist.
    std::fs::create_dir_all(&config_dir).context("Failed to create config dir")?;

    Ok(config_dir)
}

#[derive(Debug, clap::Parser)]
#[command(name = env!("CARGO_BIN_NAME"), about = "A cli to interact with https://gofile.io", version=build::CLAP_LONG_VERSION)]
struct Options {
    #[command(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, clap::Subcommand)]
enum Subcommand {
    Get(self::commands::get::Options),
    Config(self::commands::config::Options),
    Upload(self::commands::upload::Options),
    Info(self::commands::info::Options),
    GenerateCompletions(self::commands::generate_completions::Options),
}

async fn async_main(options: Options) -> anyhow::Result<()> {
    let client = gofile::Client::new();
    match options.subcommand {
        Subcommand::Get(options) => self::commands::get::exec(client, options).await?,
        Subcommand::Config(options) => self::commands::config::exec(client, options).await?,
        Subcommand::Upload(options) => self::commands::upload::exec(client, options).await?,
        Subcommand::Info(options) => self::commands::info::exec(client, options).await?,
        Subcommand::GenerateCompletions(options) => {
            self::commands::generate_completions::exec(options)?
        }
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let options = Options::parse();

    let tokio_rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    tokio_rt.block_on(async_main(options))?;

    Ok(())
}
