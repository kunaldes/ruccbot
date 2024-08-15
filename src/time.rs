use crate::{Context, Error};

use std::time::{SystemTime, UNIX_EPOCH};

/// Return the current time in seconds since the Unix epoch.
#[poise::command(slash_command)]
pub async fn time(ctx: Context<'_>) -> Result<(), Error> {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("negative time")
        .as_secs();
    ctx.send(
        poise::CreateReply::default()
            .content(time.to_string())
            .ephemeral(true),
    )
    .await?;
    Ok(())
}
