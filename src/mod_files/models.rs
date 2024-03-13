use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub enum ModFileCategory {
    Main,
    Update,
    Options,
    OldVersion,
    Misc,
}

impl Display for ModFileCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModFileCategory::Main => write!(f, "main"),
            ModFileCategory::Update => write!(f, "update"),
            ModFileCategory::Options => write!(f, "optional"),
            ModFileCategory::OldVersion => write!(f, "old_version"),
            ModFileCategory::Misc => write!(f, "miscellaneous"),
        }
    }
}

impl From<&ModFileCategory> for ModFileCategory {
    fn from(mfc: &ModFileCategory) -> Self {
        mfc.clone()
    }
}

impl From<ModFileCategory> for String {
    fn from(mfc: ModFileCategory) -> Self {
        format!("{mfc}")
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListFilesResponse {
    files: Vec<GameFileInfo>,
    file_updates: Vec<GameFileUpdatesInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameFileInfo {
    pub id: Vec<u32>,
    pub uid: u64,
    pub file_id: u32,
    pub name: String,
    pub version: String,
    pub category_id: u16,
    pub category_name: Option<String>,
    pub is_primary: bool,
    pub size: u64,
    pub file_name: String,
    pub uploaded_timestamp: u64,
    pub uploaded_time: String,
    pub mod_version: String,
    pub external_virus_scan_url: String,
    pub description: String,
    pub size_kb: u64,
    pub size_in_bytes: u64,
    pub changelog_html: Option<String>,
    pub content_preview_link: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameFileUpdatesInfo {
    pub old_file_id: u32,
    pub new_file_id: u32,
    pub old_file_name: String,
    pub new_file_name: String,
    pub uploaded_timestamp: u64,
    pub uploaded_time: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct DownloadLink {
    pub name: String,
    pub short_name: String,
    pub URI: String,
}
