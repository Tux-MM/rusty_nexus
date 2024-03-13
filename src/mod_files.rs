pub mod models;

use models::{DownloadLink, GameFileInfo, ListFilesResponse, ModFileCategory};
use reqwest::Client;
use std::{collections::HashMap, sync::Arc};

use crate::{NexusApiResult, NEXUS_API_BASE_URL};

pub struct ModFiles {
    raxios: Arc<Client>,
}

impl From<&Arc<Client>> for ModFiles {
    fn from(raxios: &Arc<Client>) -> Self {
        Self {
            raxios: raxios.clone(),
        }
    }
}

impl ModFiles {
    pub async fn list_mod_files_by_mod_id(
        &self,
        game_name: &str,
        mod_id: u32,
        category: Option<ModFileCategory>,
    ) -> NexusApiResult<ListFilesResponse> {
        let url = format!("{NEXUS_API_BASE_URL}/v1/games/{game_name}/mods/{mod_id}/files.json");
        let mut qdata = HashMap::new();

        if category.is_some() {
            qdata.insert("category",category.unwrap().to_string());
        }

        let res = self
            .raxios
            .get(url)
            .query(&qdata)
            .send()
            .await?
            .json::<ListFilesResponse>()
            .await?;

        return Ok(res);
    }

    pub async fn view_mod_file_by_id(
        &self,
        game_name: &str,
        mod_id: u32,
        file_id: u32,
    ) -> NexusApiResult<GameFileInfo> {
        let url =
            format!("{NEXUS_API_BASE_URL}/v1/games/{game_name}/mods/{mod_id}/files/{file_id}.json");
        let res = self
            .raxios
            .get(url)
            .send()
            .await?
            .json::<GameFileInfo>()
            .await?;

        return Ok(res);
    }

    pub async fn get_download_link_by_file_id_premium(
        &self,
        game_name: &str,
        mod_id: u32,
        file_id: u32,
    ) -> NexusApiResult<Vec<DownloadLink>> {
        let url = format!("{NEXUS_API_BASE_URL}/v1/games/{game_name}/mods/{mod_id}/files/{file_id}/download_link.json");

        let res = self
            .raxios
            .get(url)
            .send()
            .await?
            .json::<Vec<DownloadLink>>()
            .await?;
        return Ok(res);
    }

    pub async fn get_download_link_by_file_id(
        &self,
        game_name: &str,
        mod_id: u32,
        file_id: u32,
        key: String,
        expires: String,
    ) -> NexusApiResult<Vec<DownloadLink>> {
        let url = format!("{NEXUS_API_BASE_URL}/v1/games/{game_name}/mods/{mod_id}/files/{file_id}/download_link.json?key={key}&expires={expires}");

        let res = self
            .raxios
            .get(url)
            .send()
            .await?
            .json::<Vec<DownloadLink>>()
            .await?;
        return Ok(res);
    }
}

#[cfg(test)]
mod tests {
    use crate::NexusApi;

    use super::models::ModFileCategory;

    #[tokio::test]
    async fn test_list_mod_files_by_mod_id() {
        let api_key: &str = dotenv_codegen::dotenv!("NEXUS_API_KEY");
        let nexus_api = NexusApi::new(api_key);
        let res_no_param = nexus_api
            .mod_files
            .list_mod_files_by_mod_id("valheim", 387, None)
            .await;
        let res_with_param = nexus_api
            .mod_files
            .list_mod_files_by_mod_id("valheim", 387, Some(ModFileCategory::Main))
            .await;

        assert_ne!(true, res_no_param.is_err());
        assert_ne!(true, res_with_param.is_err());
    }

    #[tokio::test]
    async fn test_view_mod_file_by_id() {
        let api_key: &str = dotenv_codegen::dotenv!("NEXUS_API_KEY");
        let nexus_api = NexusApi::new(api_key);
        let res = nexus_api
            .mod_files
            .view_mod_file_by_id("valheim", 387, 8979)
            .await;

        assert_ne!(true, res.is_err());
    }
}
