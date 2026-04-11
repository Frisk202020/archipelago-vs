use std::{str::FromStr, time::Duration};

use poise::CreateReply;
use serenity::all::{ButtonStyle, ComponentInteractionCollector, CreateActionRow, CreateButton, CreateInteractionResponse};
use strum::{AsRefStr, EnumString};

use crate::util::{Context, Error};

pub const NO_REPLY: &'static str = "Hésite pas à répondre la prochaine fois connard";

#[derive(EnumString, AsRefStr)]
pub enum SessionInteraction {
    StartAccept,
    StartDeny,
    FinishAccept,
    FinishDeny,
    ConfirmBuild,
    DenyBuild,
} impl SessionInteraction {
    fn button(&self, disabled: bool) -> CreateButton {
        let params = match self {
            Self::StartAccept => ("TG obéi", ButtonStyle::Primary),
            Self::StartDeny => ("Pardon j'annule", ButtonStyle::Danger),
            Self::FinishAccept => ("Proceed.", ButtonStyle::Primary),
            Self::FinishDeny => ("Oups my bad", ButtonStyle::Danger),
            Self::ConfirmBuild => ("C'est tout bon", ButtonStyle::Primary),
            Self::DenyBuild => ("Att j'ai fais une boulette", ButtonStyle::Danger)
        };

        CreateButton::new(self.as_ref()).label(params.0).style(params.1).disabled(disabled)
    }

    pub async fn handle_interaction(ctx: Context<'_>, title: &str, interactions: Vec<Self>) -> Result<Option<Self>, Error> {
        let buttons = interactions.into_iter()
            .map(|x| x.button(false))
            .collect::<Vec<_>>();

        let handle = ctx.send(
            CreateReply::default()
                .content(title)
                .components(vec![CreateActionRow::Buttons(buttons.clone())])
        ).await?;

        let msg = handle.message().await?;
        let itr_opt = ComponentInteractionCollector::new(ctx)
            .message_id(msg.id)
            .timeout(Duration::from_secs(60))
            .await;

        handle.edit(
            ctx, 
            CreateReply::default()
                .content(title)
                .components(vec![
                    CreateActionRow::Buttons(
                        buttons.into_iter().map(|x| x.disabled(true)).collect()
                    )
                ])
        ).await?;

        if let Some(itr) = itr_opt {
            itr.create_response(ctx.http(), CreateInteractionResponse::Acknowledge).await?;
            Ok(Some(SessionInteraction::from_str(&itr.data.custom_id)?))
        } else {
            Ok(None)
        }
    }
}