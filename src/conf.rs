use std::fs;

#[derive(Debug)]
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
}

// 2026-11-20 01:12 - [¿?]: malas bat anjeng
pub struct Store {
    pub path: String,
    pub inner: Config,
}

impl Store {
    pub fn save(&self) {}
}
