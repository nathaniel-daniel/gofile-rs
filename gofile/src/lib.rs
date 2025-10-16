mod client;
mod model;

pub use self::client::Client;
pub(crate) use self::model::AccountResponse;
pub(crate) use self::model::ApiResponse;
pub use self::model::Page;
pub use self::model::PageChild;
pub use self::model::UploadInfo;
pub use reqwest::multipart::Part as MultipartPart;

/// Library error type
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Reqwest
    #[error("http error")]
    Reqwest(#[from] reqwest::Error),

    /// Missing token
    #[error("missing token")]
    MissingToken,
}

#[cfg(test)]
mod test {
    use super::*;

    // Search for "gofile.io/d/" on Google and pull a random working result when this dies.
    const PAGE_ID: &str = "1smiQC";

    #[tokio::test]
    async fn list_pages() {
        let client = Client::new();
        client.login_guest().await.expect("failed to log in");

        let page = client.get_page(PAGE_ID).await.expect("failed to list page");
        dbg!(page);
    }
}
