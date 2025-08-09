use crate::util::parse_page_url;
use anyhow::Context;
use anyhow::ensure;
use md5::Digest;
use md5::Md5;
use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;
use url::Url;

#[derive(Debug, argh::FromArgs)]
#[argh(
    subcommand,
    name = "download",
    description = "download from a gofile link"
)]
pub struct Options {
    #[argh(positional)]
    pub url: String,
}

pub async fn exec(client: gofile::Client, options: Options) -> anyhow::Result<()> {
    let url = Url::parse(&options.url)?;
    let id = parse_page_url(&url)?;

    client.login_guest().await.context("failed to log in")?;

    let page = client.get_page(id).await.context("failed to get page")?;
    ensure!(page.children.len() == 1);
    let child = page.children.values().next().context("missing child")?;

    let expected_md5_hash =
        base16ct::lower::decode_vec(child.md5.as_ref().context("missing md5 hash")?)?;

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

    let out_path = PathBuf::from(child.name.clone());
    let out_path_temp = nd_util::with_push_extension(&out_path, "part");

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
        ensure!(
            actual_md5_hash.as_slice() == expected_md5_hash,
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
