use std::{fs::File, io::Write};

use serde::{Deserialize, Serialize};
use serenity::model::id::{ChannelId, UserId};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Channel {
    pub welcome: ChannelId,
    pub goodbye: ChannelId,
    pub counting: ChannelId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Counting {
    pub current_number: u64,
    pub current_author: UserId,
}

impl Default for Counting {
    fn default() -> Self {
        Self {
            current_number: 1,
            current_author: UserId::default(),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub channels: Channel,
    pub counting: Counting,
}

pub struct Store {
    pub path: String,
    pub inner: Config,
}

impl Store {
    pub fn new(path: &str) -> anyhow::Result<Self> {
        let inner = match std::fs::read_to_string(path) {
            Ok(content) => serde_json::from_str::<Config>(&content)?,
            Err(_) => {
                let cfg = Config::default();
                let json = serde_json::to_string::<Config>(&cfg)?;
                let mut f = File::create_new(path)?;

                f.write_all(json.as_bytes())?;
                Config::default()
            }
        };

        Ok(Self {
            path: path.to_string(),
            inner,
        })
    }

    /// ```rust
    /// update_config(|c| c.abcd = "smth").await?;
    /// ```
    pub async fn update_config<F>(&mut self, f: F) -> anyhow::Result<()>
    where
        F: FnOnce(&mut Config),
    {
        f(&mut self.inner);
        tokio::fs::write(&self.path, serde_json::to_string::<Config>(&self.inner)?).await?;

        Ok(())
    }
}
