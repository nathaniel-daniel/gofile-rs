use crate::Config;
use crate::util::parse_page_url;
use anyhow::Context;
use anyhow::bail;
use anyhow::ensure;
use md5::Digest;
use md5::Md5;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;
use url::Url;

#[derive(Debug, clap::Parser)]
#[command(about = "Download a file or folder from a https://gofile.io link")]
pub struct Options {
    pub url: String,

    #[arg(
        long = "output",
        short = 'o',
        default_value = ".",
        help = "The output path"
    )]
    pub output: PathBuf,

    #[arg(help = "If specified, only download the child entry with this id")]
    pub child_id: Option<String>,

    #[arg(
        long = "no-append-name",
        help = "Do not append the file or folder name to the output path"
    )]
    pub no_append_name: bool,
}

async fn try_metadata<P>(path: P) -> std::io::Result<Option<std::fs::Metadata>>
where
    P: AsRef<Path>,
{
    match tokio::fs::metadata(path).await {
        Ok(metadata) => Ok(Some(metadata)),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error),
    }
}

async fn download_page_child(
    client: &gofile::Client,
    child: &gofile::PageChild,
    out_path: PathBuf,
) -> anyhow::Result<()> {
    let expected_md5_hash =
        base16ct::lower::decode_vec(child.md5.as_ref().context("missing md5 hash")?)?;

    let metadata = try_metadata(&out_path)
        .await
        .with_context(|| format!("failed to get metadata for \"{}\"", out_path.display()))?;
    match metadata {
        Some(metadata) if metadata.is_dir() => {
            bail!("output path \"{}\" is a folder", out_path.display());
        }
        Some(_metadata) => {
            // TODO: Consider validating md5 here and adding overwrite options to the cli.
            eprintln!("file exists, skipping...");
            return Ok(());
        }
        None => {}
    }
    let out_path_temp = out_path.with_added_extension("part");

    let progress_bar = indicatif::ProgressBar::new(child.size.context("missing file size")?);
    let progress_bar_style_template = "[Time = {elapsed_precise} | ETA = {eta_precise} | Speed = {bytes_per_sec}] {wide_bar} {bytes}/{total_bytes}";
    let progress_bar_style = indicatif::ProgressStyle::default_bar()
        .template(progress_bar_style_template)
        .expect("invalid progress bar style template");
    progress_bar.set_style(progress_bar_style);
    let progress_bar_tick_handle = {
        let progress_bar = progress_bar.clone();
        tokio::spawn(async move {
            while !progress_bar.is_finished() {
                progress_bar.tick();
                tokio::time::sleep(Duration::from_millis(1_000)).await;
            }
        })
    };

    let token = client.get_token()?;
    let download_url = child.link.as_ref().context("missing download url")?;
    let mut response = client
        .client
        .get(download_url)
        .header("Cookie", format!("accountToken={token}"))
        .send()
        .await?
        .error_for_status()?;
    let handle = tokio::runtime::Handle::current();
    tokio::task::spawn_blocking(move || {
        let mut hasher = Md5::new();

        let mut out_file = std::fs::File::options()
            .create(true)
            .write(true)
            .truncate(false)
            .open(&out_path_temp)?;
        out_file.try_lock()?;
        out_file.set_len(0)?;

        while let Some(chunk) = handle.block_on(response.chunk())? {
            out_file.write_all(&chunk)?;
            hasher.update(&chunk);

            let chunk_len_u64 = u64::try_from(chunk.len())?;
            progress_bar.inc(chunk_len_u64);
        }

        out_file.flush()?;
        out_file.sync_all()?;

        let actual_md5_hash = hasher.finalize();
        let actual_md5_hash_slice = actual_md5_hash.as_slice();
        ensure!(
            actual_md5_hash_slice == expected_md5_hash,
            "md5 hash mismatch"
        );

        std::fs::rename(out_path_temp, out_path)?;
        out_file.unlock()?;
        progress_bar.finish();

        anyhow::Ok(())
    })
    .await??;

    progress_bar_tick_handle.await?;

    Ok(())
}

pub async fn exec(client: gofile::Client, options: Options) -> anyhow::Result<()> {
    let config = Config::load().context("failed to load config")?;

    let url = Url::parse(&options.url)?;
    let id = parse_page_url(&url)?;

    match config.as_ref().and_then(|config| config.token.as_ref()) {
        Some(token) => client.set_token(token.clone()),
        None => client.login_guest().await?,
    }

    let page = client.get_page(id).await.context("failed to get page")?;

    match options.child_id.as_ref() {
        Some(child_id) => {
            let child = page
                .children
                .get(child_id)
                .with_context(|| format!("failed to locate child entry with id \"{child_id}\""))?;

            let mut out_path = options.output.clone();
            if !options.no_append_name {
                out_path = out_path.join(child.name.clone());
            }

            if let Some(parent) = out_path.parent() {
                tokio::fs::create_dir_all(&parent).await?;
            }

            download_page_child(&client, child, out_path).await?;
        }
        None => {
            let mut out_dir = options.output.clone();
            if !options.no_append_name {
                out_dir = out_dir.join(&page.code);
            }
            tokio::fs::create_dir_all(&out_dir).await?;

            for child in page.children.values() {
                let out_path = out_dir.join(child.name.clone());
                download_page_child(&client, child, out_path).await?;
            }
        }
    }

    Ok(())
}
