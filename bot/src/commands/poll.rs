use std::time::Duration;

use crate::util::{Context, Output};
use poise::serenity_prelude as serenity;
use ::serenity::all::{ChannelId, CreatePoll, CreatePollAnswer};

const ARCHIPELAGO_CHANNEL: u64 = 1465096954970112083;

/// Lancer un sondage pour l'archipelago du week-end prochain.
/// 
/// Ce message sera envoyé dans #archipelago
#[poise::command(slash_command)]
pub async fn poll(
    ctx: Context<'_>,
) -> Output {
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