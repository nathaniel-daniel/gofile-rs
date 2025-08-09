use time::OffsetDateTime;

/// The info on an upload
#[derive(Debug, serde::Deserialize)]
pub struct UploadInfo {
    /// The time this was created.
    #[serde(rename = "createTime", with = "time::serde::timestamp")]
    pub create_time: OffsetDateTime,

    /// The page where this file can be downloaded.
    #[serde(rename = "downloadPage")]
    pub download_page: String,

    /// The guest token.
    ///
    /// Only included if this file was uploaded using a guest token.
    #[serde(rename = "guestToken")]
    pub guest_token: Option<String>,

    /// The file id?
    pub id: String,

    /// The md5 hash
    pub md5: String,

    /// The mime type
    pub mimetype: String,

    /// The modification time
    #[serde(rename = "modTime", with = "time::serde::timestamp")]
    pub mod_time: OffsetDateTime,

    /// The file name
    pub name: String,

    /// The parent folder id?
    #[serde(rename = "parentFolder")]
    pub parent_folder: String,

    /// The parent folder code
    #[serde(rename = "parentFolderCode")]
    pub parent_folder_code: String,

    /// The servers the file is stored on?
    pub servers: Vec<String>,

    /// The file size in bytes
    pub size: u64,

    /// The file type?
    ///
    /// Valid:
    /// "file"
    #[serde(rename = "type")]
    pub kind: String,
}
