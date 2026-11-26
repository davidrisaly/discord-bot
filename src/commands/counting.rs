//! for checking if counting goes right

use super::{CONFIG, FrameworkContext, Message, Result};
use crate::config::{Counting, Store};
use serenity::{model::id::UserId, prelude::CacheHttp};

pub async fn counting(ctx: FrameworkContext<'_>, message: &Message) -> Result<()> {
    let mut guard = CONFIG.write().await;
    let cfg = &guard.inner;

    let current_author = cfg.counting.current_author;
    let current_number = cfg.counting.current_number;
    let current_number_default = Counting::default().current_number;

    let message_u64 = message.content.parse::<u64>()?;
    let cache_http = &ctx.serenity_context.http();
    let warn_same_author = &format!(
        "<@{}>, you're already done this!\nCounter resets to {}.",
        message.author.id, &current_number_default
    );
    let warn_wrong_num = &format!(
        "Wrong number! <@{}>, The next number is {}.\nCounter resets to {}.",
        message.author.id, current_number, &current_number_default
    );

    if message_u64 == current_number && message.author.id != current_author {
        success(&mut guard, message).await?;
        message.react(&ctx.serenity_context.http(), '✅').await?;
    } else if message_u64 != current_number {
        reset(&mut guard, current_number_default).await?;
        message.channel_id.say(cache_http, warn_wrong_num).await?;
    } else if message.author.id == current_author {
        reset(&mut guard, current_number_default).await?;
        message.channel_id.say(cache_http, warn_same_author).await?;
    }

    Ok(())
}

async fn success(guard: &mut Store, message: &Message) -> Result<()> {
    guard
        .update_config(|c| c.counting.current_number += 1)
        .await?;
    guard
        .update_config(|c| c.counting.current_author = message.author.id)
        .await?;
    Ok(())
}

async fn reset(guard: &mut Store, num_reset: u64) -> Result<()> {
    guard
        .update_config(|c| c.counting.current_number = num_reset)
        .await?;
    guard
        .update_config(|c| c.counting.current_author = UserId::default())
        .await?;
    Ok(())
}
