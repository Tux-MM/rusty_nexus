pub mod models;

use self::models::{Message, TrackedModsGetResponse, ValidateResponse};
use crate::NexusApiResult;
use raxios::{map_string, ContentType, Raxios, RaxiosOptions};
use std::{collections::HashMap, sync::Arc};

pub struct Nexus {
    raxios: Arc<Raxios>,
}

impl From<&Arc<Raxios>> for Nexus {
    fn from(raxios: &Arc<Raxios>) -> Self {
        Self {
            raxios: raxios.clone(),
        }
    }
}

impl Nexus {
    pub async fn validate(&self) -> NexusApiResult<ValidateResponse> {
        let url = "v1/users/validate.json";
        let resp = self.raxios.get::<ValidateResponse>(url, None).await?;
        Ok(resp.body.unwrap())
    }

    pub async fn tracked_mods(&self) -> NexusApiResult<TrackedModsGetResponse> {
        let url = "v1/user/tracked_mods.json";
        let resp = self.raxios.get::<TrackedModsGetResponse>(url, None).await?;
        Ok(resp.body.unwrap())
    }

    pub async fn track_mod(&self, game: &str, mod_id: u32) -> NexusApiResult<Message> {
        let url = format!("v1/user/tracked_mods.json?domain_name={game}");
        let data = map_string! {mod_id : mod_id};
        let res = self
            .raxios
            .post::<Message, HashMap<String, String>>(
                &url,
                Some(data),
                Some(RaxiosOptions {
                    content_type: Some(ContentType::UrlEncoded),
                    ..Default::default()
                }),
            )
            .await?;
        Ok(res.body.unwrap())
    }

    pub async fn untrack_mod(&self, game: &str, mod_id: u32) -> NexusApiResult<Message> {
        let url = format!("v1/user/tracked_mods.json?domain_name={game}");
        let data = map_string! {mod_id : mod_id};
        let res = self
            .raxios
            .delete::<HashMap<String, String>, Message>(
                &url,
                Some(data),
                Some(RaxiosOptions {
                    content_type: Some(ContentType::UrlEncoded),
                    ..Default::default()
                }),
            )
            .await?;
        Ok(res.body.unwrap())
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
            true,
            "{resp:?}"
        );
        let resp = nexus_api.nexus.untrack_mod("skyrimspecialedition", 1).await;
        assert_eq!(resp.is_ok(), true, "{resp:?}");
    }
}
