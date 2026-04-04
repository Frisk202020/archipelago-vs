use std::sync::Arc;

use serenity::all::{ComponentInteraction, CreateInteractionResponse, CreateInteractionResponseMessage, Http, Mentionable};

use crate::{commands::session::data::Data, util::Output};

pub(crate) async fn handle_replace_session_data(
    x: &ComponentInteraction, http: Arc<Http>
) -> Output {
    let data = Data::default();
    data.write()?;
    
    x.create_response(http, CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().content("Allez hop c'est parti")
    )).await?;

    Ok(())
}

pub(crate) async fn handle_replace_finish_time(
    x: &ComponentInteraction, http: Arc<Http>
) -> Output {
    let mut data = Data::get()?;

    let target = x.message.mentions.get(0);
    if let Some(target) = target {
        let end = data.finish(target.display_name(), true).unwrap();
        data.write()?;

        x.create_response(http, CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().content(format!("GG {}, tu as fins en {}", target.mention(), data.display_elapsed(&end)))
        )).await?;
    } else {
        x.create_response(http, CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().content("J'iai perdu le fil désolé :crying_face:")
        )).await?;
    }

    Ok(())
}