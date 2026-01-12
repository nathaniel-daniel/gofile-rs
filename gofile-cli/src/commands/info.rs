use crate::Config;
use crate::util::parse_page_url;
use anyhow::Context;
use url::Url;

#[derive(Debug, Default, Copy, Clone, clap::ValueEnum)]
pub enum OutputFormat {
    #[default]
    Human,

    Json,
}

#[derive(Debug, clap::Parser)]
#[command(about = "Get the info from a https://gofile.io link")]
pub struct Options {
    pub url: String,

    #[arg(
        long = "output-format",
        default_value_t = Default::default(),
        value_enum,
    )]
    pub output_format: OutputFormat,
}

fn output_human(page: &gofile::Page) {
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
}

fn output_json(page: &gofile::Page) -> anyhow::Result<()> {
    let stdout = std::io::stdout();
    let lock = stdout.lock();
    serde_json::to_writer(lock, page)?;
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
    match options.output_format {
        OutputFormat::Human => output_human(&page),
        OutputFormat::Json => output_json(&page)?,
    }

    Ok(())
}
