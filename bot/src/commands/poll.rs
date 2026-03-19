use std::time::Duration;

use crate::util::{Context, Error};
use poise::serenity_prelude as serenity;
use ::serenity::all::{ChannelId, CreatePoll, CreatePollAnswer};

const ARCHIPELAGO_CHANNEL: u64 = 1465096954970112083;

#[poise::command(slash_command)]
pub(crate) async fn poll(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let poll = CreatePoll::new()
        .question("Archipelago ce week-end ?")
        .answers(
            vec!(
                "Vendredi soir",
                "Samedi aprem",
                "Dimanche aprem"
            )
            .into_iter()
            .map(|x| CreatePollAnswer::new().text(x))
            .collect()
        )
        .duration(Duration::from_secs(3600 * 24 * 5))
        .allow_multiselect();
    let message = serenity::CreateMessage::new().poll(poll);
    
    ChannelId::new(ARCHIPELAGO_CHANNEL).send_message(ctx.http(), message).await?;
    ctx.say("C'est fait, mais t'as perdu").await?;
    Ok(())
}