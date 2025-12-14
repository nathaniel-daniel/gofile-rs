use anyhow::Context;

use tokio::io::AsyncWriteExt;

const DEFAULT_CONFIG: &str = r#"# Your account api token. (Optional)
# token = "YOUR TOKEN HERE"
"#;

#[derive(Debug, clap::Parser)]
#[command(about = "Manage the CLI config")]
pub struct Options {
    #[command(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
    Edit(EditOptions),
}

#[derive(Debug, clap::Parser)]
#[command(about = "edit the config")]
pub struct EditOptions {}

pub async fn exec(_client: gofile::Client, options: Options) -> anyhow::Result<()> {
    match options.subcommand {
        Subcommand::Edit(_options) => {
            let config_dir = crate::get_config_dir().context("failed to get config dir")?;

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
