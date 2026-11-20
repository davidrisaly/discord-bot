mod cmd;
mod conf;
mod env;
mod lv;
mod tests;

use poise::serenity_prelude as serenity;
use serenity::FullEvent::{GuildMemberAddition, GuildMemberRemoval, Ready};
use serenity::model::id::ChannelId;

async fn event_handler(
    ctx: poise::FrameworkContext<'_, cmd::Data, anyhow::Error>,
    event: &serenity::FullEvent,
) -> anyhow::Result<()> {
    if let serenity::FullEvent::Ready { data_about_bot } = event {
        println!("{}", data_about_bot.user.name);
    }

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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let token = env::var("DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: cmd::cmds(),
            event_handler: |ctx, event| Box::pin(event_handler(ctx, event)),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(cmd::Data {})
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await?;
    client.start().await?;

    Ok(())
}
