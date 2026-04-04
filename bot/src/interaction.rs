use std::{str::FromStr, sync::Arc};

use serenity::all::{ButtonStyle, CreateActionRow, CreateButton, CreateInteractionResponse, EditMessage, Http, Interaction, Mentionable};
use strum::{AsRefStr, EnumString};

use crate::{commands::{handle_replace_finish_time, handle_replace_session_data}, util::{DEVELOPER, Output}};

#[derive(EnumString, AsRefStr, PartialEq)]
pub(crate) enum ArchibotButtonInteraction {
    ReplaceSessionAccept,
    ReplaceSessionDecline,
    ReplaceFinishTimeAccept,
    ReplaceFinishTimeDecline
} impl ArchibotButtonInteraction {
    pub(crate) fn button(&self) -> CreateButton {
        match self {
            Self::ReplaceSessionAccept => CreateButton::new(self.as_ref()).label("TG obéi"),
            Self::ReplaceSessionDecline => CreateButton::new(self.as_ref()).label("Pardon j'annule").style(ButtonStyle::Danger),
            Self::ReplaceFinishTimeAccept => CreateButton::new(self.as_ref()).label("Yes my bad"),
            Self::ReplaceFinishTimeDecline => CreateButton::new(self.as_ref()).label("Oups retour arrière").style(ButtonStyle::Danger)
        }
    }
}

pub(crate) async fn handle(http: Arc<Http>, x: &Interaction) -> Output {
    match x {
        Interaction::Component(x) => {
            if let Ok(variant) = ArchibotButtonInteraction::from_str(&x.data.custom_id) {
                match variant {
                    ArchibotButtonInteraction::ReplaceSessionAccept | ArchibotButtonInteraction::ReplaceSessionDecline => {
                        let mut message = x.message.to_owned();
                        message.edit(http.clone(), EditMessage::default().content(&message.content).components(vec![
                            CreateActionRow::Buttons(vec![
                                ArchibotButtonInteraction::ReplaceSessionAccept.button().disabled(true), 
                                ArchibotButtonInteraction::ReplaceSessionDecline.button().disabled(true)
                            ])
                        ])).await?;

                        if variant == ArchibotButtonInteraction::ReplaceSessionAccept {
                            handle_replace_session_data(x, http).await?;
                        } else {
                            x.create_response(http, CreateInteractionResponse::Acknowledge).await?;
                        }
                    }, ArchibotButtonInteraction::ReplaceFinishTimeAccept | ArchibotButtonInteraction::ReplaceFinishTimeDecline => {
                        let mut message = x.message.to_owned();
                        message.edit(http.clone(), EditMessage::default().content(&message.content).components(vec![
                            CreateActionRow::Buttons(vec![
                                ArchibotButtonInteraction::ReplaceFinishTimeAccept.button().disabled(true), 
                                ArchibotButtonInteraction::ReplaceFinishTimeDecline.button().disabled(true)
                            ])
                        ])).await?;

                        if variant == ArchibotButtonInteraction::ReplaceFinishTimeAccept {
                            handle_replace_finish_time(x, http).await?;
                        } else {
                            x.create_response(http, CreateInteractionResponse::Acknowledge).await?;
                        }
                    }
                } 
            } else {
                x.message.reply(http, format!("Bon ba cet abruti de {} m'a pas dit quoi faire", DEVELOPER.mention())).await?;
            }
        },
        _ => {}
    }

    Ok(())
}