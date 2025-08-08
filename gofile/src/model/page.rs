use std::collections::HashMap;

/// A download page
#[derive(Debug, serde::Deserialize)]
pub struct Page {
    #[serde(rename = "childrenCount")]
    pub children_count: u64,
    pub code: String,
    #[serde(rename = "createTime")]
    pub create_time: u64,
    pub id: String,
    pub children: HashMap<String, PageChild>,
    #[serde(rename = "totalSize")]
    pub total_size: u64,
    #[serde(rename = "modTime")]
    pub mod_time: u64,
    pub name: String,
    pub public: bool,
    #[serde(rename = "totalDownloadCount")]
    pub total_download_count: u64,
}

#[derive(Debug, serde::Deserialize)]
pub struct PageChild {
    #[serde(rename = "downloadCount")]
    pub download_count: Option<u64>,
    #[serde(rename = "createTime")]
    pub create_time: u64,
    #[serde(rename = "modTime")]
    pub mod_time: u64,
    /// The url to download from
    pub link: Option<String>,
    /// The md5 hex hash
    pub md5: Option<String>,
    /// The size
    pub size: Option<u64>,
    /// The name of the file
    pub name: String,
}
