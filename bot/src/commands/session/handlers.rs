use std::sync::Arc;

use serenity::all::{ComponentInteraction, CreateInteractionResponse, CreateInteractionResponseMessage, Http};

use crate::{commands::session::data::Data, util::Error};

pub(crate) async fn handle_replace_session_data(
    x: &ComponentInteraction, http: Arc<Http>
) -> Result<(), Error> {
    let data = Data::default();
    data.write()?;
    
    x.create_response(http, CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().content("Allez hop c'est parti")
    )).await?;

    Ok(())
}