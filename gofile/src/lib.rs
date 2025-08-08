mod client;
mod model;

pub use self::client::Client;
pub(crate) use self::model::AccountResponse;
pub(crate) use self::model::ApiResponse;
pub use self::model::Page;
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

    #[tokio::test]
    async fn list_pages() {
        let page_id = "BQC8o2";

        let client = Client::new();
        client.login_guest().await.expect("failed to log in");

        let page = client.get_page(page_id).await.expect("failed to list page");
        dbg!(page);
    }
}
