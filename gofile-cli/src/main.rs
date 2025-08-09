mod commands;
mod config;
mod util;

pub use self::config::Config;
use anyhow::Context;
use etcetera::AppStrategy;
use etcetera::AppStrategyArgs;
use std::path::PathBuf;

pub fn get_config_dir() -> anyhow::Result<PathBuf> {
    let app_strategy = etcetera::choose_app_strategy(AppStrategyArgs {
        app_name: "gofile-cli".into(),
        author: "".into(),
        top_level_domain: "".into(),
    })?;

    let config_dir = app_strategy.config_dir();

    // Create config dir if it does not exist.
    std::fs::create_dir_all(&config_dir).context("failed to create config dir")?;

    Ok(config_dir)
}

#[derive(Debug, argh::FromArgs)]
#[argh(description = "a cli to interact with gofile")]
struct Options {
    #[argh(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, argh::FromArgs)]
#[argh(subcommand)]
enum Subcommand {
    Download(self::commands::download::Options),
    Config(self::commands::config::Options),
    Upload(self::commands::upload::Options),
    Info(self::commands::info::Options),
}

async fn async_main(options: Options) -> anyhow::Result<()> {
    let client = gofile::Client::new();
    match options.subcommand {
        Subcommand::Download(options) => self::commands::download::exec(client, options).await?,
        Subcommand::Config(options) => self::commands::config::exec(client, options).await?,
        Subcommand::Upload(options) => self::commands::upload::exec(client, options).await?,
        Subcommand::Info(options) => self::commands::info::exec(client, options).await?,
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let options: Options = argh::from_env();

    let tokio_rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    tokio_rt.block_on(async_main(options))?;

    Ok(())
}
