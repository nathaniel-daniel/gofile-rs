use crate::AccountResponse;
use crate::ApiResponse;
use crate::Error;
use crate::Page;
use reqwest::header::AUTHORIZATION;
use std::sync::Arc;

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36";

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
    fn set_token(&self, token: String) {
        *self.state.token.lock().expect("token poisoned") = Some(token);
    }

    /// Get the token.
    ///
    /// Should not need to be used under normal circumstances.
    pub fn get_token(&self) -> Result<String, Error> {
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
        // wt is fairly constant, embedded in a js file.
        let wt = "4fd6sg89d7s6";
        let page = 1;
        let page_size = 1000;
        let url = format!(
            "https://api.gofile.io/contents/{id}?wt={wt}&contentFilter=&page={page}&pageSize={page_size}&sortField=name&sortDirection=1"
        );
        let token = self.get_token()?;
        let api_response: ApiResponse<Page> = self
            .client
            .get(url)
            .header(AUTHORIZATION, format!("Bearer {token}"))
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

/// The client state
#[derive(Debug)]
struct ClientState {
    token: std::sync::Mutex<Option<String>>,
}
