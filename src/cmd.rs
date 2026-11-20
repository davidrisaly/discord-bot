use crate::{conf::Config, env, serenity};
use anyhow::{Error as E, Result};
use std::sync::{LazyLock, RwLock};
use unb_api::structs::UnbelievaBoat;

pub struct U {}
pub type Context<'a> = poise::Context<'a, U, E>;

/// Provides function expansion to UnbelievaBoat with the help of UnbelievaBoat API.
// TODO: Make this bot's own economy later.
static _UNB: LazyLock<UnbelievaBoat> =
    LazyLock::new(|| UnbelievaBoat::new(env::var("UNB_TOKEN").as_str()).unwrap());

// TODO: Variabel ini hanya menyimpan data di RAM yang hilang saat aplikasi restart.
//       Buat hal ini supaya punya backup persisten seperti di disk, contohnya dengan
//       membuat file konfigurasi JSON atau bahkan pada database seperti SQLite.
static CONFIG: LazyLock<RwLock<Config>> = LazyLock::new(|| RwLock::new(Config::new()));

pub fn cmds() -> Vec<poise::Command<U, E>> {
    vec![config()]
}

#[poise::command(
    slash_command,
    subcommands("welcome", "goodbye", "counting", "debug"),
    subcommand_required
)]
async fn config(_ctx: Context<'_>) -> Result<()> {
    Ok(())
}

#[poise::command(slash_command)]
async fn welcome(
    ctx: Context<'_>,
    #[description = "A dedicated channel for welcoming new members"]
    #[channel_types("Text")]
    channel: serenity::GuildChannel,
) -> Result<()> {
    ctx.say(format!("{}", channel.id)).await?;

    {
        let mut guard = CONFIG.write().unwrap();
        guard.welcome_channel = channel.id.into();
    }

    Ok(())
}

#[poise::command(slash_command)]
async fn goodbye(
    ctx: Context<'_>,
    #[description = "A dedicated channel for saying goodbye for leaving members"]
    #[channel_types("Text")]
    channel: serenity::GuildChannel,
) -> Result<()> {
    ctx.say(format!("{}", channel.id)).await?;

    {
        let mut guard = CONFIG.write().unwrap();
        guard.goodbye_channel = channel.id.into();
    }

    Ok(())
}

#[poise::command(slash_command)]
async fn counting(
    ctx: Context<'_>,
    #[description = "A dedicated channel for counting"]
    #[channel_types("Text")]
    channel: serenity::GuildChannel,
) -> Result<()> {
    ctx.say(format!("{}", channel.id)).await?;

    {
        let mut guard = CONFIG.write().unwrap();
        guard.counting_channel = channel.id.into();
    }

    Ok(())
}

#[poise::command(slash_command)]
async fn debug(ctx: Context<'_>) -> Result<()> {
    let text = {
        let cfg = CONFIG.read().unwrap();
        format!("```rust\n{:#?}\n```", *cfg)
    };
    ctx.say(text).await?;
    Ok(())
}
