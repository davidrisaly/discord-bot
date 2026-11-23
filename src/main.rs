mod commands;
mod config;
mod env;
mod leveling;
mod tests;

use poise::serenity_prelude as serenity;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let token = env::var("DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::GUILD_MEMBERS
        | serenity::GatewayIntents::GUILD_MESSAGES;
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands::cmds(),
            event_handler: |ctx, event| {
                Box::pin(commands::event_handler::event_handler(ctx, event))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(commands::U {})
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await?;
    client.start().await?;

    Ok(())
}
