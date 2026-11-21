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
    ctx.say(format!("{}", channel.id)).await?;

    {
        let mut guard = CONFIG.write().await;
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
        let mut guard = CONFIG.write().await;
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
        let mut guard = CONFIG.write().await;
        guard.counting_channel = channel.id.into();
    }

    Ok(())
}

#[poise::command(slash_command)]
async fn debug(ctx: Context<'_>) -> Result<()> {
    let text = {
        let cfg = CONFIG.read().await;
        format!("```rust\n{:#?}\n```", *cfg)
    };
    ctx.say(text).await?;
    Ok(())
}
