pub mod admin;

use crate::{conf::Config, env::var, serenity};
use anyhow::{Error as E, Result};
use serenity::{
    FullEvent::{GuildMemberAddition, GuildMemberRemoval, Ready},
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

// TODO: Variabel ini hanya menyimpan data di RAM yang hilang saat aplikasi restart.
//       Buat hal ini supaya punya backup persisten seperti di disk, contohnya dengan
//       membuat file konfigurasi JSON atau bahkan pada database seperti SQLite.
static CONFIG: LazyLock<RwLock<Config>> = LazyLock::new(|| RwLock::new(Config::default()));

pub async fn event_handler(
    ctx: poise::FrameworkContext<'_, U, E>,
    event: &serenity::FullEvent,
) -> Result<()> {
    match event {
        Ready { data_about_bot } => {
            println!("{}", data_about_bot.user.name);
        }
        GuildMemberAddition { new_member } => {
            ChannelId::new(1570146478272553061)
                .say(
                    &ctx.serenity_context.http,
                    format!("Welcome, {}!", new_member.user.name),
                )
                .await?;
            new_member
                .add_role(&ctx.serenity_context.http, 1566272833246732328)
                .await?;
        }
        GuildMemberRemoval { user, .. } => {
            ChannelId::new(1570146550921953461)
                .say(
                    &ctx.serenity_context.http,
                    format!("Goodbye, {}!", user.name),
                )
                .await?;
        }
        _ => {}
    }

    Ok(())
}

pub fn cmds() -> Vec<poise::Command<U, E>> {
    vec![admin::config()]
}
