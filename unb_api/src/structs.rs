use serde::{Deserialize, Serialize};

pub struct UnbelievaBoat {
    pub client: reqwest::Client,
}

#[derive(Deserialize)]
pub struct GuildLeaderboard {
    pub users: Vec<UserBalance>,
    pub total_pages: i32,
}

#[derive(Deserialize)]
pub struct UserBalance {
    pub rank: String,
    pub user_id: String,
    pub cash: i64,
    pub bank: i64,
    pub total: i64,
}

#[derive(Serialize)]
pub struct UpdateBalance {
    pub cash: i32,
    pub bank: i32,
    pub reason: &'static str,
}
