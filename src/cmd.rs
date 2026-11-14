use crate::serenity::{CreateEmbed, CreateEmbedAuthor};
use crate::{api, env};
use num_format::{Locale, ToFormattedString};
use std::sync::LazyLock;

pub struct Data {} // User data, which is stored and accessible in all command invocations
pub type Context<'a> = poise::Context<'a, Data, anyhow::Error>;

static UNB: LazyLock<api::UnbelievaBoat> =
    LazyLock::new(|| api::UnbelievaBoat::new(env::var("UNB_TOKEN").as_str()).unwrap());

pub fn cmds() -> Vec<poise::Command<Data, anyhow::Error>> {
    vec![bal()]
}

#[poise::command(slash_command, guild_only)]
pub async fn bal(ctx: Context<'_>) -> anyhow::Result<()> {
    let res = UNB
        .get_user_balance(ctx.guild_id().unwrap().into(), ctx.author().id.into())
        .await?;

    let embed = CreateEmbed::new()
        .author(
            CreateEmbedAuthor::new(
                res.user_id
                    .parse::<crate::serenity::UserId>()?
                    .to_user(&ctx)
                    .await?
                    .name,
            )
            .icon_url(
                res.user_id
                    .parse::<crate::serenity::UserId>()?
                    .to_user(&ctx)
                    .await?
                    .face(),
            ),
        )
        .description(format!("Leaderboard Rank: {}", res.rank))
        .color(0x3498DB)
        .field(
            "Cash",
            format!("⭓{}", res.cash.to_formatted_string(&Locale::en)),
            true,
        )
        .field(
            "Bank",
            format!("⭓{}", res.bank.to_formatted_string(&Locale::en)),
            true,
        )
        .field(
            "Total",
            format!("⭓{}", res.total.to_formatted_string(&Locale::en)),
            true,
        );

    let reply = poise::CreateReply::default().embed(embed);

    ctx.send(reply).await?;

    Ok(())
}
