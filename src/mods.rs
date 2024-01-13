pub mod models;

use crate::NexusApiResult;
use models::{ModEndorsementResult, ModInfoResponse, Period, UpdatedModInfo};
use raxios::{map_string, Raxios, RaxiosOptions};
use std::{collections::HashMap, sync::Arc};

pub struct Mods {
    raxios: Arc<Raxios>,
}

impl From<&Arc<Raxios>> for Mods {
    fn from(raxios: &Arc<Raxios>) -> Self {
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
        let url = format!("/v1/games/{game_name}/mods/updated.json");

        period.to_string();

        let to_return = self
            .raxios
            .get::<Vec<UpdatedModInfo>>(
                &url,
                Some(RaxiosOptions {
                    params: Some(map_string! {period : period}),
                    ..Default::default()
                }),
            )
            .await?;

        Ok(to_return.body.unwrap())
    }

    pub async fn get_changelog_by_mod_id(
        &self,
        game_name: &str,
        mod_id: u32,
    ) -> NexusApiResult<HashMap<String, Vec<String>>> {
        let url = format!("/v1/games/{game_name}/mods/{mod_id}/changelogs.json");

        let response = self
            .raxios
            .get::<HashMap<String, Vec<String>>>(&url, None)
            .await?;

        return Ok(response.body.unwrap());
    }

    pub async fn get_lastest_10_mods_by_game(
        &self,
        game_name: &str,
    ) -> NexusApiResult<Vec<ModInfoResponse>> {
        let url = format!("/v1/games/{game_name}/mods/latest_added.json");

        let response = self.raxios.get::<Vec<ModInfoResponse>>(&url, None).await?;

        return Ok(response.body.unwrap());
    }

    pub async fn get_latest_10_updated_mods_by_game(
        &self,
        game_name: &str,
    ) -> NexusApiResult<Vec<ModInfoResponse>> {
        let url = format!("/v1/games/{game_name}/mods/latest_updated.json");

        let res = self.raxios.get::<Vec<ModInfoResponse>>(&url, None).await?;

        return Ok(res.body.unwrap());
    }

    pub async fn get_top_10_trending_mods_by_game(
        &self,
        game_name: &str,
    ) -> NexusApiResult<Vec<ModInfoResponse>> {
        let url = format!("v1/games/{game_name}/mods/trending.json");
        let response = self.raxios.get::<Vec<ModInfoResponse>>(&url, None).await?;

        return Ok(response.body.unwrap());
    }

    pub async fn get_mod_info_for_game(
        &self,
        mod_id: u32,
        game_name: &str,
    ) -> NexusApiResult<ModInfoResponse> {
        let url = format!("v1/games/{game_name}/mods/{mod_id}.json");
        let response = self.raxios.get::<ModInfoResponse>(&url, None).await?;

        return Ok(response.body.unwrap());
    }

    pub async fn endorse_mod_by_mod_id(
        &self,
        game_name: &str,
        mod_id: u32,
        mod_version: &str,
    ) -> NexusApiResult<ModEndorsementResult> {
        let url = format!("v1/games/{game_name}/mods/{mod_id}/endorse.json");
        let data = map_string! {version : mod_version};

        let response = self
            .raxios
            .post::<ModEndorsementResult, HashMap<String, String>>(
                &url,
                Some(data),
                Some(RaxiosOptions {
                    content_type: Some(raxios::ContentType::UrlEncoded),
                    ..Default::default()
                }),
            )
            .await?;

        return Ok(response.body.unwrap());
    }

    pub async fn remove_mod_endorsement_by_mod_id(
        &self,
        game_name: &str,
        mod_id: u32,
        mod_version: &str,
    ) -> NexusApiResult<ModEndorsementResult> {
        let url = format!("v1/games/{game_name}/mods/{mod_id}/abstain.json");
        let data = map_string! {version : mod_version};
        let res = self
            .raxios
            .post::<ModEndorsementResult, HashMap<String, String>>(
                &url,
                Some(data),
                Some(RaxiosOptions {
                    content_type: Some(raxios::ContentType::UrlEncoded),
                    ..Default::default()
                }),
            )
            .await?;

        return Ok(res.body.unwrap());
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
