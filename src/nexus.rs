pub mod models;

use self::models::{Message, TrackedModsGetResponse, ValidateResponse};
use crate::{NexusApiResult, NEXUS_API_BASE_URL};
use reqwest::Client;
use std::{collections::HashMap, sync::Arc};

pub struct Nexus {
    raxios: Arc<Client>,
}

impl From<&Arc<Client>> for Nexus {
    fn from(raxios: &Arc<Client>) -> Self {
        Self {
            raxios: raxios.clone(),
        }
    }
}

impl Nexus {
    pub async fn validate(&self) -> NexusApiResult<ValidateResponse> {
        let url = format!("{NEXUS_API_BASE_URL}/v1/users/validate.json");
        let resp = self
            .raxios
            .get(url)
            .send()
            .await?
            .json::<ValidateResponse>()
            .await?;
        Ok(resp)
    }

    pub async fn tracked_mods(&self) -> NexusApiResult<TrackedModsGetResponse> {
        let url = format!("{NEXUS_API_BASE_URL}/v1/user/tracked_mods.json");
        let resp = self
            .raxios
            .get(url)
            .send()
            .await?
            .json::<TrackedModsGetResponse>()
            .await?;
        Ok(resp)
    }

    pub async fn track_mod(&self, game: &str, mod_id: u32) -> NexusApiResult<Message> {
        let url = format!("{NEXUS_API_BASE_URL}/v1/user/tracked_mods.json");
        let mut form_data = HashMap::new();
        form_data.insert("domain_name", game);
        let mut data = HashMap::new();
        data.insert("mod_id", mod_id);
        let res = self
            .raxios
            .post(&url)
            .query(&form_data)
            .form(&data)
            .send()
            .await?
            .json::<Message>()
            .await?;
        Ok(res)
    }

    pub async fn untrack_mod(&self, game: &str, mod_id: u32) -> NexusApiResult<Message> {
        let url = format!("{NEXUS_API_BASE_URL}/v1/user/tracked_mods.json?domain_name={game}");
        let mut data = HashMap::new();
        data.insert("mod_id", mod_id);
        let res = self
            .raxios
            .delete(url)
            .json(&data)
            .send()
            .await?
            .json::<Message>()
            .await?;
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::models::TrackedModsCommon;
    use crate::NexusApi;
    #[tokio::test]
    async fn test_validate() {
        let api_key: &str = dotenv_codegen::dotenv!("NEXUS_API_KEY");
        let nexus_api = NexusApi::new(api_key);
        let resp = nexus_api.nexus.validate().await.unwrap();
        assert_eq!(api_key, resp.key);
    }
    #[tokio::test]
    async fn test_track_mods() {
        let api_key: &str = dotenv_codegen::dotenv!("NEXUS_API_KEY");
        let nexus_api = NexusApi::new(api_key);
        let resp = nexus_api.nexus.track_mod("skyrimspecialedition", 1).await;
        assert_eq!(resp.is_ok(), true, "{resp:?}");
        let resp = nexus_api.nexus.tracked_mods().await.unwrap();
        assert_eq!(
            resp.contains(&TrackedModsCommon {
                domain_name: "skyrimspecialedition".into(),
                mod_id: 1
            }),
            true
        );
        let resp = nexus_api.nexus.untrack_mod("skyrimspecialedition", 1).await;
        assert_eq!(resp.is_ok(), true);
    }
}
