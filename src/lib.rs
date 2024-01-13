pub mod mod_files;
pub mod mods;
pub mod nexus;

use std::sync::Arc;

use mod_files::ModFiles;
use mods::Mods;
use nexus::Nexus;
use raxios::{map_string, Raxios, RaxiosConfig};

pub type NexusApiResult<T> = anyhow::Result<T>;

const NEXUS_API_BASE_URL: &'static str = "https://api.nexusmods.com/";

pub struct NexusApi {
    pub mods: Mods,
    pub mod_files: ModFiles,
    pub nexus: Nexus,
}

impl NexusApi {
    pub fn new(api_key: &str) -> Self {
        let default_headers = map_string! {
            apikey : api_key
        };

        let raxios = Raxios::new(
            NEXUS_API_BASE_URL,
            Some(RaxiosConfig {
                headers: Some(default_headers),
                ..Default::default()
            }),
        )
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