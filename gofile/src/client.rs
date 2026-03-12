use crate::AccountResponse;
use crate::ApiResponse;
use crate::Error;
use crate::MultipartPart;
use crate::Page;
use crate::UploadInfo;
use reqwest::header::AUTHORIZATION;
use reqwest::multipart::Form;
use sha2::Digest;
use sha2::Sha256;
use std::sync::Arc;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36";
const LANGUAGE: &str = "en-US";

fn unix_epoch_secs() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time is before the unix epoch")
        .as_secs_f64()
}

/// See: https://github.com/yt-dlp/yt-dlp/issues/16117#issuecomment-4011609858
fn generate_website_token(token: &str) -> String {
    let time_4 = (unix_epoch_secs() / f64::from(60 * 60 * 4)) as u64;
    let hash_salt = "gf2026x";

    let data = format!("{USER_AGENT}::{LANGUAGE}::{token}::{time_4}::{hash_salt}");

    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let hash = hasher.finalize();

    base16ct::lower::encode_string(&hash)
}

/// The client state
#[derive(Debug)]
struct ClientState {
    token: std::sync::Mutex<Option<(String, String)>>,
}

/// A client
#[derive(Debug, Clone)]
pub struct Client {
    /// A http client
    pub client: reqwest::Client,

    /// The client state
    state: Arc<ClientState>,
}

impl Client {
    /// Make a new client.
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .build()
            .expect("failed to build client");
        let state = Arc::new(ClientState {
            token: std::sync::Mutex::new(None),
        });
        Self { client, state }
    }

    /// Set the token.
    pub fn set_token(&self, token: String) {
        let website_token = generate_website_token(&token);
        *self.state.token.lock().expect("token poisoned") = Some((token, website_token));
    }

    /// Get the token and website token.
    ///
    /// Should not need to be used under normal circumstances.
    pub fn get_tokens(&self) -> Result<(String, String), Error> {
        self.state
            .token
            .lock()
            .expect("token poisoned")
            .clone()
            .ok_or(Error::MissingToken)
    }

    /// Login as a guest.
    pub async fn login_guest(&self) -> Result<(), Error> {
        let url = "https://api.gofile.io/accounts";
        let api_response: ApiResponse<AccountResponse> = self
            .client
            .post(url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        self.set_token(api_response.data.token);

        Ok(())
    }

    /// Get a page.
    pub async fn get_page(&self, id: &str) -> Result<Page, Error> {
        let (token, website_token) = self.get_tokens()?;
        let page = 1;
        let page_size = 1000;
        let url = format!(
            "https://api.gofile.io/contents/{id}?contentFilter=&page={page}&pageSize={page_size}&sortField=name&sortDirection=1"
        );
        let request = self
            .client
            .get(url)
            .header(AUTHORIZATION, format!("Bearer {token}"));
        let api_response: ApiResponse<Page> = request
            .header("X-Website-Token", website_token)
            .header("X-BL", LANGUAGE)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(api_response.data)
    }

    /// Upload a file
    pub async fn upload(&self, file: MultipartPart) -> Result<UploadInfo, Error> {
        let form = Form::new().part("file", file);

        let url = "https://upload.gofile.io/uploadfile";
        let (token, _) = self.get_tokens()?;
        let api_response: ApiResponse<UploadInfo> = self
            .client
            .post(url)
            .header(AUTHORIZATION, format!("Bearer {token}"))
            .multipart(form)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(api_response.data)
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}
