pub mod mod_files;
pub mod mods;
pub mod nexus;

use std::sync::Arc;

use mod_files::ModFiles;
use mods::Mods;
use nexus::Nexus;
use reqwest::{header::HeaderMap, ClientBuilder};

pub type NexusApiResult<T> = anyhow::Result<T>;

const NEXUS_API_BASE_URL: &'static str = "https://api.nexusmods.com";

pub struct NexusApi {
    pub mods: Mods,
    pub mod_files: ModFiles,
    pub nexus: Nexus,
}

impl NexusApi {
    pub fn new(api_key: &str) -> Self {
        let mut def_headers = HeaderMap::new();
        def_headers.insert("apikey", api_key.parse().unwrap());

        let raxios = ClientBuilder::new()
            .default_headers(def_headers)
            .user_agent("RUSTY_NEXUS/1.0")
            .build()
            .unwrap();

        let raxios = Arc::new(raxios);
        let mods = Mods::from(&raxios);
        let mod_files = ModFiles::from(&raxios);
        let nexus = Nexus::from(&raxios);

        Self {
            mods,
            mod_files,
            nexus,
        }
    }
}
