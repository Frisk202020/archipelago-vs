use std::{str::FromStr, sync::Arc};

use serenity::all::{Context, Http, Interaction};
use strum::{AsRefStr, EnumString};

use crate::util::{Error};

#[derive(EnumString, AsRefStr)]
pub(crate) enum ComponentInteraction {
    SessionDataNotFoundOk,
    SessionDataNotFoundDecline,
} 

pub(crate) async fn handle(http: Arc<Http>, x: &Interaction) -> Result<(), Error> {
    match x {
        Interaction::Component(x) => {
            if let Ok(variant) = ComponentInteraction::from_str(&x.data.custom_id) {
                match variant {
                    ComponentInteraction::SessionDataNotFoundOk => { x.message.reply(http, "Ok").await; },
                    ComponentInteraction::SessionDataNotFoundDecline => { }
                }
            } else {

            }
        },
        _ => { }
    };

    Ok(())
}