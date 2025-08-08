use anyhow::Context;
use etcetera::AppStrategy;
use etcetera::AppStrategyArgs;
use tokio::io::AsyncWriteExt;

const DEFAULT_CONFIG: &str = r#"# The account api token.
# token = "YOUR TOKEN HERE"
"#;

#[derive(Debug, argh::FromArgs)]
#[argh(subcommand, name = "config", description = "Manage the CLI config")]
pub struct Options {
    #[argh(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, argh::FromArgs)]
#[argh(subcommand)]
pub enum Subcommand {
    Edit(EditOptions),
}

#[derive(Debug, argh::FromArgs)]
#[argh(subcommand, name = "edit", description = "edit the config")]
pub struct EditOptions {}

pub async fn exec(_client: gofile::Client, options: Options) -> anyhow::Result<()> {
    match options.subcommand {
        Subcommand::Edit(_options) => {
            let app_strategy = etcetera::choose_app_strategy(AppStrategyArgs {
                app_name: "gofile-cli".into(),
                author: "".into(),
                top_level_domain: "".into(),
            })?;

            let config_dir = app_strategy.config_dir();
            tokio::fs::create_dir_all(&config_dir).await?;

            let config_path = config_dir.join("config.toml");
            match tokio::fs::File::create_new(&config_path).await {
                Ok(mut file) => {
                    file.write_all(DEFAULT_CONFIG.as_bytes()).await?;
                }
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
                Err(error) => {
                    return Err(error).context("failed to create default config file");
                }
            }

            opener::open(&config_path)?;
        }
    }
    Ok(())
}
