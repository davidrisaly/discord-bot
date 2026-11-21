use serde::{Deserialize, Serialize};
use serenity::model::id::ChannelId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub welcome_channel: ChannelId,
    pub goodbye_channel: ChannelId,
    pub counting_channel: ChannelId,
}

pub struct Store {
    pub path: String,
    pub inner: Config,
}

impl Store {
    pub fn new(path: &str) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let cfg = serde_json::from_str::<Config>(&content)?;

        Ok(Self {
            path: path.to_string(),
            inner: cfg,
        })
    }

    /// ```rust update_config(|c| c.abcd = "smth");```
    pub async fn update_config<F>(&mut self, f: F) -> anyhow::Result<()>
    where
        F: FnOnce(&mut Config),
    {
        f(&mut self.inner);
        tokio::fs::write(&self.path, serde_json::to_string::<Config>(&self.inner)?).await?;

        Ok(())
    }
}
