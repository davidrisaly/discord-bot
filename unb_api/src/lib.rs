pub mod structs;

use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};
use structs::*;

impl UnbelievaBoat {
    pub fn new(token: &str) -> anyhow::Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(AUTHORIZATION, HeaderValue::from_str(token.trim())?);

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self { client })
    }

    pub async fn get_guild_leaderboard(&self, guild_id: u64) -> reqwest::Result<GuildLeaderboard> {
        self
            .client
            .get(format!(
                "https://unbelievaboat.com/api/v1/guilds/{}",
                guild_id
            ))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_user_balance(
        &self,
        guild_id: u64,
        user_id: u64,
    ) -> reqwest::Result<UserBalance> {
        self
            .client
            .get(format!(
                "https://unbelievaboat.com/api/v1/guilds/{}/users/{}",
                guild_id, user_id
            ))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn edit_user_balance(
        &self,
        guild_id: u64,
        user_id: u64,
        data: &UpdateBalance,
    ) -> reqwest::Result<()> {
        self.client
            .patch(format!(
                "https://unbelievaboat.com/api/v1/guilds/{}/users/{}",
                guild_id, user_id
            ))
            .json(data)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    pub async fn set_user_balance(
        &self,
        guild_id: u64,
        user_id: u64,
        data: UpdateBalance,
    ) -> reqwest::Result<()> {
        self.client
            .put(format!(
                "https://unbelievaboat.com/api/v1/guilds/{}/users/{}",
                guild_id, user_id
            ))
            .json(&data)
            .send()
            .await?;

        Ok(())
    }
}
