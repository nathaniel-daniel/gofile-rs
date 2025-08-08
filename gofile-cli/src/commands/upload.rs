use crate::Config;
use anyhow::Context;
use anyhow::bail;
use std::path::PathBuf;

#[derive(Debug, argh::FromArgs)]
#[argh(subcommand, name = "upload", description = "Upload a file to gofile")]
pub struct Options {
    #[argh(positional, description = "the path to the file to upload")]
    pub path: PathBuf,

    #[argh(
        switch,
        long = "use-guest",
        description = "force the use of a guest token"
    )]
    pub use_guest: bool,
}

pub async fn exec(client: gofile::Client, options: Options) -> anyhow::Result<()> {
    let config = Config::load().context("failed to load config")?;

    if !options
        .path
        .try_exists()
        .context("failed to check if file exists")?
    {
        bail!("file \"{}\" does not exist", options.path.display());
    }
    let file = gofile::MultipartPart::file(options.path).await?;

    if options.use_guest {
        client
            .login_guest()
            .await
            .context("failed to log in as a guest")?;
    } else {
        let missing_token_message = "Missing token. Use the \"--use-guest\" flag to use a guest token or add a token to your config";
        let token = config
            .as_ref()
            .and_then(|config| config.token.as_ref())
            .context(missing_token_message)?;
        client.set_token(token.clone());
    }
    let upload_info = client.upload(file).await.context("failed to upload file")?;

    println!("{}", upload_info.download_page);

    Ok(())
}
