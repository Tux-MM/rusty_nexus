pub mod models;

use crate::{NexusApiResult, NEXUS_API_BASE_URL};
use models::{ModEndorsementResult, ModInfoResponse, Period, UpdatedModInfo};
use reqwest::Client;
use std::{collections::HashMap, sync::Arc};

pub struct Mods {
    raxios: Arc<Client>,
}

impl From<&Arc<Client>> for Mods {
    fn from(raxios: &Arc<Client>) -> Self {
        Self {
            raxios: raxios.clone(),
        }
    }
}

impl Mods {
    pub async fn get_updated_mods_by_game(
        &self,
        period: Period,
        game_name: &str,
    ) -> NexusApiResult<Vec<UpdatedModInfo>> {
        let url = format!("{NEXUS_API_BASE_URL}/v1/games/{game_name}/mods/updated.json");

        let mut data = HashMap::new();
        data.insert("period", period.to_string());
        let to_return = self
            .raxios
            .get(url)
            .json(&data)
            .send()
            .await?
            .json::<Vec<UpdatedModInfo>>()
            .await?;
        Ok(to_return)
    }

    pub async fn get_changelog_by_mod_id(
        &self,
        game_name: &str,
        mod_id: u32,
    ) -> NexusApiResult<HashMap<String, Vec<String>>> {
        let url =
            format!("{NEXUS_API_BASE_URL}/v1/games/{game_name}/mods/{mod_id}/changelogs.json");

        let response = self
            .raxios
            .get(url)
            .send()
            .await?
            .json::<HashMap<String, Vec<String>>>()
            .await?;

        return Ok(response);
    }

    pub async fn get_lastest_10_mods_by_game(
        &self,
        game_name: &str,
    ) -> NexusApiResult<Vec<ModInfoResponse>> {
        let url = format!("{NEXUS_API_BASE_URL}/v1/games/{game_name}/mods/latest_added.json");

        let response = self
            .raxios
            .get(url)
            .send()
            .await?
            .json::<Vec<ModInfoResponse>>()
            .await?;

        return Ok(response);
    }

    pub async fn get_latest_10_updated_mods_by_game(
        &self,
        game_name: &str,
    ) -> NexusApiResult<Vec<ModInfoResponse>> {
        let url = format!("{NEXUS_API_BASE_URL}/v1/games/{game_name}/mods/latest_updated.json");

        let res = self
            .raxios
            .get(url)
            .send()
            .await?
            .json::<Vec<ModInfoResponse>>()
            .await?;

        return Ok(res);
    }

    pub async fn get_top_10_trending_mods_by_game(
        &self,
        game_name: &str,
    ) -> NexusApiResult<Vec<ModInfoResponse>> {
        let url = format!("{NEXUS_API_BASE_URL}/v1/games/{game_name}/mods/trending.json");
        let response = self
            .raxios
            .get(url)
            .send()
            .await?
            .json::<Vec<ModInfoResponse>>()
            .await?;

        return Ok(response);
    }

    pub async fn get_mod_info_for_game(
        &self,
        mod_id: u32,
        game_name: &str,
    ) -> NexusApiResult<ModInfoResponse> {
        let url = format!("{NEXUS_API_BASE_URL}/v1/games/{game_name}/mods/{mod_id}.json");
        let response = self
            .raxios
            .get(url)
            .send()
            .await?
            .json::<ModInfoResponse>()
            .await?;

        return Ok(response);
    }

    pub async fn endorse_mod_by_mod_id(
        &self,
        game_name: &str,
        mod_id: u32,
        mod_version: &str,
    ) -> NexusApiResult<ModEndorsementResult> {
        let url = format!("{NEXUS_API_BASE_URL}/v1/games/{game_name}/mods/{mod_id}/endorse.json");
        let mut data = HashMap::new();
        data.insert("version", mod_version);

        let response = self
            .raxios
            .post(url)
            .json(&data)
            .send()
            .await?
            .json::<ModEndorsementResult>()
            .await?;

        return Ok(response);
    }

    pub async fn remove_mod_endorsement_by_mod_id(
        &self,
        game_name: &str,
        mod_id: u32,
        mod_version: &str,
    ) -> NexusApiResult<ModEndorsementResult> {
        let url = format!("{NEXUS_API_BASE_URL}/v1/games/{game_name}/mods/{mod_id}/abstain.json");
        let mut data = HashMap::new();
        data.insert("version", mod_version);
        let res = self
            .raxios
            .post(url)
            .json(&data)
            .send()
            .await?
            .json::<ModEndorsementResult>()
            .await?;

        return Ok(res);
    }
}

#[cfg(test)]
mod tests {
    use super::models::Period;
    use crate::NexusApi;

    #[tokio::test]
    async fn test_get_updated_mods_by_game() {
        let api_key: &str = dotenv_codegen::dotenv!("NEXUS_API_KEY");
        let nexus_api = NexusApi::new(api_key);
        let period = Period::Week;

        let result = nexus_api
            .mods
            .get_updated_mods_by_game(period, "valheim")
            .await;

        assert_ne!(true, result.is_err());
    }

    #[tokio::test]
    async fn test_get_changelog_by_mod_id() {
        let api_key: &str = dotenv_codegen::dotenv!("NEXUS_API_KEY");
        let nexus_api = NexusApi::new(api_key);

        let result = nexus_api.mods.get_changelog_by_mod_id("valheim", 387).await;

        assert_ne!(true, result.is_err());
    }

    #[tokio::test]
    async fn test_get_last_10_mods_by_name() {
        let api_key: &str = dotenv_codegen::dotenv!("NEXUS_API_KEY");
        let nexus_api = NexusApi::new(api_key);

        let result = nexus_api.mods.get_lastest_10_mods_by_game("valheim").await;

        assert_ne!(true, result.is_err());
    }

    #[tokio::test]
    async fn test_get_latest_10_updated_mods_by_game() {
        let api_key: &str = dotenv_codegen::dotenv!("NEXUS_API_KEY");

        let nexus_api = NexusApi::new(api_key);

        let res = nexus_api
            .mods
            .get_latest_10_updated_mods_by_game("valheim")
            .await;

        assert_ne!(true, res.is_err());
    }

    #[tokio::test]
    async fn test_get_top_10_trending_mods_by_game() {
        let api_key: &str = dotenv_codegen::dotenv!("NEXUS_API_KEY");
        let nexus_api = NexusApi::new(api_key);
        let res = nexus_api
            .mods
            .get_latest_10_updated_mods_by_game("valheim")
            .await;

        assert_ne!(true, res.is_err());
    }

    #[tokio::test]
    async fn test_get_mod_info_for_game() {
        let api_key: &str = dotenv_codegen::dotenv!("NEXUS_API_KEY");
        let nexus_api = NexusApi::new(api_key);
        let res = nexus_api.mods.get_mod_info_for_game(387, "valheim").await;

        assert_ne!(true, res.is_err());
    }

    #[tokio::test]
    async fn test_endorse_mod_by_mod_id() {
        let api_key: &str = dotenv_codegen::dotenv!("NEXUS_API_KEY");
        let nexus_api = NexusApi::new(api_key);
        let res = nexus_api
            .mods
            .endorse_mod_by_mod_id("valheim", 387, "0")
            .await;

        assert_ne!(true, res.is_err());
    }

    #[tokio::test]
    async fn test_remove_mod_endorsement_by_mod_id() {
        let api_key: &str = dotenv_codegen::dotenv!("NEXUS_API_KEY");
        let nexus_api = NexusApi::new(api_key);
        let res = nexus_api
            .mods
            .remove_mod_endorsement_by_mod_id("valheim", 387, "0")
            .await;

        assert_ne!(true, res.is_err());
    }
}
