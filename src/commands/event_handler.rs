use super::*;

pub async fn event_handler(
    ctx: poise::FrameworkContext<'_, U, E>,
    event: &serenity::FullEvent,
) -> Result<()> {
    match event {
        Ready { data_about_bot } => {
            if let Some(discriminator) = data_about_bot.user.discriminator {
                println!("{}#{} is online", data_about_bot.user.name, discriminator);
            } else {
                println!("{} is online", data_about_bot.user.name)
            }
        }
        Message { new_message }
            if new_message.channel_id == CONFIG.read().await.inner.channels.counting =>
        {
            counting(new_message);
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
