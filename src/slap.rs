use crate::{Context, Error};

/// Slap Ian Mitchell
#[poise::command(slash_command)]
pub async fn slap(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("_slaps <@90339695967350784> around a bit with a large trout_")
        .await?;
    Ok(())
}
