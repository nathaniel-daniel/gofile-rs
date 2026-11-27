use std::collections::HashMap;
use time::OffsetDateTime;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
pub enum PageChildKind {
    #[serde(rename = "file")]
    File,

    #[serde(rename = "folder")]
    Folder,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct PageChild {
    /// The number of times this has been downloaded.
    ///
    /// Not present for folders.
    #[serde(rename = "downloadCount")]
    pub download_count: Option<u64>,

    #[serde(rename = "createTime", with = "time::serde::timestamp")]
    pub create_time: OffsetDateTime,

    #[serde(rename = "modTime", with = "time::serde::timestamp")]
    pub mod_time: OffsetDateTime,

    /// The url to download from.
    ///
    /// Not present for folders.
    pub link: Option<String>,

    /// The md5 hash, as a hex string.
    ///
    /// Not present for folders.
    pub md5: Option<String>,

    /// The size of the file, in bytes.
    ///
    /// Not present for folders.
    pub size: Option<u64>,

    /// The name of the file.
    pub name: String,

    /// The type of the child.
    ///
    /// Either "file" or "folder".
    #[serde(rename = "type")]
    pub kind: PageChildKind,

    /// The folder code.
    ///
    /// This may be used just like any other folder code.
    /// This shows up in the url as `https://gofile.io/d/{code}`.
    ///
    /// Not present for files.
    pub code: Option<String>,

    /// The id of the entry.
    ///
    /// This uniquely identifies the file or folder on the entire website.
    pub id: String,

    /// The number of children in this folder.
    ///
    /// Not present for files.
    #[serde(rename = "childrenCount")]
    pub children_count: Option<u64>,
}

/// A download page
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Page {
    /// The number of children.
    ///
    /// This can be absent in some cases?
    #[serde(rename = "childrenCount")]
    pub children_count: Option<u64>,

    /// The folder code.
    ///
    /// This shows up in the url as `https://gofile.io/d/{code}`.
    pub code: String,

    #[serde(rename = "createTime", with = "time::serde::timestamp")]
    pub create_time: OffsetDateTime,

    /// The id of the page.
    ///
    /// This uniquely identifies the folder on the entire website.
    pub id: String,

    /// ?
    ///
    /// This can be absent in some cases?
    pub children: Option<HashMap<String, PageChild>>,

    /// ?
    ///
    /// This can be absent in some cases?
    #[serde(rename = "totalSize")]
    pub total_size: Option<u64>,

    #[serde(rename = "modTime", with = "time::serde::timestamp")]
    pub mod_time: OffsetDateTime,
    pub name: String,

    /// ?
    ///
    /// This can be absent in some cases?
    pub public: Option<bool>,

    /// ?
    ///
    /// This can be absent in some cases?
    #[serde(rename = "totalDownloadCount")]
    pub total_download_count: Option<u64>,

    /// Whether the page is frozen.
    ///
    /// If frozen, this is present and true.
    #[serde(rename = "isFrozen")]
    pub is_frozen: Option<bool>,
}
