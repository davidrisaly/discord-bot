use serde::{Deserialize, Serialize};
use tokio::fs;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub welcome_channel: u64,
    pub goodbye_channel: u64,
    pub counting_channel: u64,
}

impl Config {
    pub fn new() -> Self {
        Self {
            welcome_channel: 0,
            goodbye_channel: 0,
            counting_channel: 0,
        }
    }

    pub fn from(self, mut store: Store) {
        store.inner = self
    }
}

pub struct Store {
    pub path: String,
    pub inner: Config,
}

impl Store {
    pub fn save(&self) {}
    pub async fn load(&self) -> anyhow::Result<Config> {
        let content = fs::read_to_string(&self.path).await?;
        Ok(serde_json::from_str::<Config>(&content)?)
    }

    // update_config(|c| c.abcd = "smth")
    pub async fn update_config<F>(&mut self, f: F) -> anyhow::Result<()>
    where
        F: FnOnce(&mut Config),
    {
        f(&mut self.inner);

        fs::write(&self.path, serde_json::to_string::<Config>(&self.inner)?).await?;

        Ok(())
    }
}
