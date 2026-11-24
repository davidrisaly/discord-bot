//! for checking if counting goes right

use super::{CONFIG, FrameworkContext, Message, Result};
use crate::config::Counting;
use serenity::{model::id::UserId, prelude::CacheHttp};

pub async fn counting(ctx: FrameworkContext<'_>, message: &Message) -> Result<()> {
    let mut guard = CONFIG.write().await;
    let cfg = &guard.inner;

    let current_number = cfg.counting.current_number;
    let current_author = cfg.counting.current_author;

    if message.content.parse::<u64>()? == current_number && message.author.id != current_author {
        guard
            .update_config(|c| c.counting.current_number += 1)
            .await?;
        guard
            .update_config(|c| c.counting.current_author = message.author.id)
            .await?;
        message.react(&ctx.serenity_context.http(), '✅').await?;
    } else {
        cfg.channels
            .counting
            .say(
                &ctx.serenity_context.http(),
                format!(
                    "Wrong number! <@{}>. The next number is {}",
                    message.author.id,
                    current_number + 1
                ),
            )
            .await?;
        guard
            .update_config(|c| c.counting.current_number = Counting::default().current_number)
            .await?;
        guard
            .update_config(|c| c.counting.current_author = UserId::default())
            .await?;
    }

    Ok(())
}
