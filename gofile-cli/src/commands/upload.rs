use crate::Config;
use anyhow::Context as _;
use anyhow::bail;
use std::path::Path;
use std::path::PathBuf;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use std::time::Duration;
use tokio::io::AsyncRead;
use tokio::io::ReadBuf;

#[derive(Debug, clap::Parser)]
#[command(about = "Upload a file to https://gofile.io")]
pub struct Options {
    #[arg(help = "The path to the file to upload")]
    pub path: PathBuf,

    #[arg(long = "use-guest", help = "Force the use of a guest token")]
    pub use_guest: bool,
}

pin_project_lite::pin_project! {
    struct UploadProgressTracker {
        progress_bar: indicatif::ProgressBar,
        position: u64,
        len: u64,
        #[pin]
        file: tokio::fs::File,
    }
}

impl UploadProgressTracker {
    pub async fn new_multipart_part(
        path: &Path,
    ) -> anyhow::Result<(gofile::MultipartPart, indicatif::ProgressBar)> {
        let file_name = path
            .file_name()
            .map(|filename| filename.to_string_lossy().into_owned());
        let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
        let mime = mime_guess::from_ext(extension).first_or_octet_stream();

        let file = tokio::fs::File::open(path).await?;
        let metadata = file.metadata().await?;
        let len = metadata.len();

        let progress_bar = indicatif::ProgressBar::new(len);
        let progress_bar_style_template = "[Time = {elapsed_precise} | ETA = {eta_precise} | Speed = {bytes_per_sec}] {wide_bar} {bytes}/{total_bytes}";
        let progress_bar_style = indicatif::ProgressStyle::default_bar()
            .template(progress_bar_style_template)
            .expect("invalid progress bar style template");
        progress_bar.set_style(progress_bar_style);

        {
            let progress_bar = progress_bar.clone();
            tokio::spawn(async move {
                while !progress_bar.is_finished() {
                    progress_bar.tick();
                    tokio::time::sleep(Duration::from_millis(1_000)).await;
                }
            });
        }

        let tracker = Self {
            progress_bar: progress_bar.clone(),
            position: 0,
            len,
            file,
        };

        let stream = reqwest::Body::wrap_stream(tokio_util::io::ReaderStream::new(tracker));
        let mut part =
            gofile::MultipartPart::stream_with_length(stream, len).mime_str(mime.essence_str())?;
        if let Some(file_name) = file_name {
            part = part.file_name(file_name);
        }

        Ok((part, progress_bar))
    }
}

impl AsyncRead for UploadProgressTracker {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let this = self.as_mut().project();

        let start = buf.filled().len();
        let result = this.file.poll_read(cx, buf);
        let end = buf.filled().len();
        let change = u64::try_from(end - start).unwrap();
        *this.position += change;
        self.progress_bar.inc(change);

        result
    }
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
    let (file, progress_bar) = UploadProgressTracker::new_multipart_part(&options.path).await?;

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
    progress_bar.finish();

    println!("Url: {}", upload_info.download_page);
    println!("Id: {}", upload_info.id);
    println!("Size: {}", upload_info.size);
    println!("Parent Folder Id: {}", upload_info.parent_folder);
    println!("Parent Folder Code: {}", upload_info.parent_folder_code);
    if let Some(guest_token) = upload_info.guest_token {
        println!("Guest Token: {guest_token}");
    }

    Ok(())
}
