use anyhow::Context;
use anyhow::ensure;
use url::Url;

pub fn parse_page_url(url: &Url) -> anyhow::Result<&str> {
    ensure!(url.host_str() == Some("gofile.io"));

    let mut path_iter = url.path_segments().context("missing path")?;
    ensure!(path_iter.next() == Some("d"));
    let id = path_iter.next().context("missing id")?;
    ensure!(path_iter.next().is_none());

    Ok(id)
}
