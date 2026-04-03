use poise::CreateReply;
use serenity::all::CreateActionRow;

use crate::{commands::session::data::Data, interaction::ArchibotButtonInteraction, util::{Context, Error}};

#[poise::command(slash_command)]
pub(crate) async fn start_session(ctx: Context<'_>) -> Result<(), Error> {
    let data = if let Ok(data) = Data::get() { data } else {
        let data = Data::default();
        data.write()?;
        ctx.say("C'est tout bon").await?;
        return Ok(());
    };

    let title = if data.active {
        "Pour info une session est en cours selon mes données, et procéder va tout foutre à la poubelle. T'es sur de ton coup ?"
    } else {
        "Fais Gaffe mon gars, j'ai déjà une session (inactive) en stock, ducoup c'est bon je peux la supprime ?"
    };

    ctx.send(
        CreateReply::default()
            .content(title)
            .components(vec![
                CreateActionRow::Buttons(vec![
                    ArchibotButtonInteraction::ReplaceSessionAccept.button(),
                    ArchibotButtonInteraction::ReplaceSessionDecline.button()
                ])
            ])
    ).await?;

    Ok(())
}