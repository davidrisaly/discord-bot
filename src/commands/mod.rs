pub mod admin;
pub mod event_handler;
pub mod moderator;

use crate::{config::Store, counting::counting, env::var, serenity};
use anyhow::{Error as E, Result};
use serenity::{
    FullEvent::{GuildMemberAddition, GuildMemberRemoval, Message, Ready},
    model::id::ChannelId,
};
use std::sync::LazyLock;
use tokio::sync::RwLock;
use unb_api::structs::UnbelievaBoat;

pub struct U {}
pub type Context<'a> = poise::Context<'a, U, E>;

/// Provides function expansion to UnbelievaBoat with the help of UnbelievaBoat API.
// TODO: Make this bot's own economy later.
static _UNB: LazyLock<UnbelievaBoat> =
    LazyLock::new(|| UnbelievaBoat::new(var("UNB_TOKEN")).unwrap());

static CONFIG: LazyLock<RwLock<Store>> =
    LazyLock::new(|| RwLock::new(Store::new("data/config.json").unwrap()));

pub fn cmds() -> Vec<poise::Command<U, E>> {
    vec![admin::config()]
}
