use crate::config::Config;

use super::*;

#[poise::command(
    slash_command,
    subcommands("welcome", "goodbye", "counting", "debug"),
    subcommand_required,
    required_permissions = "ADMINISTRATOR"
)]
pub async fn config(_ctx: Context<'_>) -> Result<()> {
    Ok(())
}

#[poise::command(slash_command)]
async fn welcome(
    ctx: Context<'_>,
    #[description = "A dedicated channel for welcoming new members"]
    #[channel_types("Text")]
    channel: serenity::GuildChannel,
) -> Result<()> {
    let mut guard = CONFIG.write().await;

    ctx.say(format!(
        "Welcome channel set to <#{}> successfully",
        channel.id
    ))
    .await?;
    guard
        .update_config(|c| c.channels.welcome = channel.id)
        .await?;

    Ok(())
}

#[poise::command(slash_command)]
async fn goodbye(
    ctx: Context<'_>,
    #[description = "A dedicated channel for saying goodbye for leaving members"]
    #[channel_types("Text")]
    channel: serenity::GuildChannel,
) -> Result<()> {
    let mut guard = CONFIG.write().await;

    ctx.say(format!(
        "Goodbye channel set to <#{}> successfully",
        channel.id
    ))
    .await?;
    guard
        .update_config(|c| c.channels.goodbye = channel.id)
        .await?;

    Ok(())
}

#[poise::command(slash_command)]
async fn counting(
    ctx: Context<'_>,
    #[description = "A dedicated channel for counting"]
    #[channel_types("Text")]
    channel: serenity::GuildChannel,
) -> Result<()> {
    let mut guard = CONFIG.write().await;

    ctx.say(format!(
        "Counting channel set to <#{}> successfully",
        channel.id
    ))
    .await?;
    guard
        .update_config(|c| c.channels.counting = channel.id)
        .await?;

    Ok(())
}

#[poise::command(slash_command)]
async fn debug(ctx: Context<'_>) -> Result<()> {
    let text = {
        let cfg = serde_json::to_string_pretty::<Config>(&CONFIG.read().await.inner)?;
        format!("```json\n{}\n```", cfg)
    };
    ctx.say(text).await?;
    Ok(())
}
