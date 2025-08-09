use crate::util::parse_page_url;
use anyhow::Context;
use url::Url;

#[derive(Debug, argh::FromArgs)]
#[argh(
    subcommand,
    name = "info",
    description = "Get the info from a gofile link"
)]
pub struct Options {
    #[argh(positional)]
    pub url: String,
}

pub async fn exec(client: gofile::Client, options: Options) -> anyhow::Result<()> {
    let url = Url::parse(&options.url)?;
    let id = parse_page_url(&url)?;

    client.login_guest().await?;

    let page = client.get_page(id).await.context("failed to get page")?;
    for (id, page) in page.children.iter() {
        println!("Id: {id}");
        println!("Name: {}", page.name);
        println!("Create Time: {}", page.create_time);
        println!("Mod Time: {}", page.mod_time);
        if let Some(size) = page.size {
            println!("Size: {size} bytes");
        }
        if let Some(md5) = page.md5.as_ref() {
            println!("Md5: {md5}");
        }
        if let Some(download_count) = page.download_count.as_ref() {
            println!("Download Count: {download_count}");
        }
        if let Some(link) = page.link.as_ref() {
            println!("Link: {link}");
        }
        println!();
    }

    Ok(())
}
