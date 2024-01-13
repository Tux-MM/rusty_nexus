use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ValidateResponse {
    pub user_id: u64,
    pub key: String,
    pub name: String,
    pub email: String,
    pub profile_url: String,
    pub is_premium: bool,
    pub is_supporter: bool,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct TrackedModsCommon {
    pub mod_id: u32,
    pub domain_name: String,
}

pub type TrackedModsGetResponse = Vec<TrackedModsCommon>;

#[derive(Deserialize, Serialize, Debug)]
pub struct Message {
    pub message: String
}
