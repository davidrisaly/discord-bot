use reqwest::header::{ACCEPT, AUTHORIZATION, HeaderMap, HeaderValue};
use std::collections::HashMap;

#[derive(Clone, serde::Deserialize)]
pub struct UserBalance {
    pub rank: String,
    pub user_id: String,
    pub cash: i64,
    pub bank: i64,
    pub total: i64,
}

pub struct UnbelievaBoat {
    pub client: reqwest::Client,
}

impl UnbelievaBoat {
    pub fn new(token: &str) -> anyhow::Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(AUTHORIZATION, HeaderValue::from_str(token)?);

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self { client })
    }

    pub async fn get_guild_leaderboard(
        &self,
        guild_id: u64,
    ) -> reqwest::Result<HashMap<String, serde_json::Value>> {
        Ok(self
            .client
            .get(format!(
                "https://unbelievaboat.com/api/v1/guilds/{}",
                guild_id
            ))
            .send()
            .await?
            .json::<HashMap<String, serde_json::Value>>()
            .await?)
    }

    pub async fn get_user_balance(
        &self,
        guild_id: u64,
        user_id: u64,
    ) -> reqwest::Result<UserBalance> {
        Ok(self
            .client
            .get(format!(
                "https://unbelievaboat.com/api/v1/guilds/{}/users/{}",
                guild_id, user_id
            ))
            .send()
            .await?
            .json::<UserBalance>()
            .await?)
    }
}
