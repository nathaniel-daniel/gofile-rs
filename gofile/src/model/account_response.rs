/// Response for making an account
#[derive(Debug, serde::Deserialize)]
pub(crate) struct AccountResponse {
    /// The auth token to use for api calls
    pub token: String,
}
