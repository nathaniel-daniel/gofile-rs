mod account_response;
mod page;
mod upload_info;

pub(crate) use self::account_response::AccountResponse;
pub use self::page::Page;
pub use self::page::PageChild;
pub use self::upload_info::UploadInfo;

#[derive(Debug, serde::Deserialize)]
pub(crate) struct ApiResponse<T> {
    // /// The status
    // pub status: String,
    /// The data
    pub data: T,
}
