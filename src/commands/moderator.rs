use serenity::model::guild::Member;

use super::{Context, Result};

#[poise::command(slash_command)]
pub async fn ban(ctx: Context<'_>, member: Member, reason: String) -> Result<()> {
    ctx.say(format!(
        "User <@{}> has been banned with reason: {}",
        member.user.id, &reason
    ))
    .await?;
    member.ban_with_reason(ctx.http(), 7, &reason).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn kick(ctx: Context<'_>, member: Member, reason: String) -> Result<()> {
    ctx.say(format!(
        "User <@{}> has been kicked with reason: {}",
        member.user.id, reason
    ))
    .await?;
    member.kick_with_reason(ctx.http(), &reason).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn warn(ctx: Context<'_>) -> Result<()> {
    Ok(())
}
