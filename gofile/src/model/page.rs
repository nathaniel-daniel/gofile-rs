use std::collections::HashMap;
use time::OffsetDateTime;

/// A download page
#[derive(Debug, serde::Deserialize)]
pub struct Page {
    /// The number of children
    #[serde(rename = "childrenCount")]
    pub children_count: u64,

    /// The folder code.
    ///
    /// This shows up in the url as `https://gofile.io/d/{code}`.
    pub code: String,

    #[serde(rename = "createTime", with = "time::serde::timestamp")]
    pub create_time: OffsetDateTime,
    pub id: String,
    pub children: HashMap<String, PageChild>,
    #[serde(rename = "totalSize")]
    pub total_size: u64,
    #[serde(rename = "modTime", with = "time::serde::timestamp")]
    pub mod_time: OffsetDateTime,
    pub name: String,
    pub public: bool,
    #[serde(rename = "totalDownloadCount")]
    pub total_download_count: u64,
}

#[derive(Debug, serde::Deserialize)]
pub struct PageChild {
    #[serde(rename = "downloadCount")]
    pub download_count: Option<u64>,
    #[serde(rename = "createTime", with = "time::serde::timestamp")]
    pub create_time: OffsetDateTime,
    #[serde(rename = "modTime", with = "time::serde::timestamp")]
    pub mod_time: OffsetDateTime,
    /// The url to download from
    pub link: Option<String>,
    /// The md5 hex hash
    pub md5: Option<String>,
    /// The size
    pub size: Option<u64>,
    /// The name of the file
    pub name: String,
}
