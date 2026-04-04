use poise::CreateReply;
use serenity::all::{CreateActionRow, Mentionable, User};

use crate::{commands::session::data::Data, interaction::ArchibotButtonInteraction, util::{Context, Output}};

const SESSION_DATA_MISSING_MSG: &'static str = "Commence peut-être par lancer une session hein.";

#[poise::command(slash_command)]
pub(crate) async fn start_session(ctx: Context<'_>) -> Output {
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

#[poise::command(slash_command)]
pub(crate) async fn finish(ctx: Context<'_>, player: Option<User>) -> Output {
    if let Ok(mut data) = Data::get() { 
        if !data.active {
            ctx.say("La session est terminée, recommencez une nouvelle partie avec `start_session`.").await?;
            return Ok(()); 
        }

        let user = player.as_ref().unwrap_or(ctx.author());
        if let Some(end) = data.finish(user.display_name(), false) {
            data.write()?;
            ctx.say(format!("GG {}, tu as fini en {} !", user.mention(), data.display_elapsed(&end))).await?;
        } else {
            ctx.send(
                CreateReply::default()
                    .content(format!("Mais... {} avait déjà finit, on remplace par ce nouveau temps ?", user.mention()))
                    .components(vec![
                        CreateActionRow::Buttons(vec![
                            ArchibotButtonInteraction::ReplaceFinishTimeAccept.button(),
                            ArchibotButtonInteraction::ReplaceFinishTimeDecline.button()
                        ])
                    ])
            ).await?;
        }
    } else {
        ctx.say(SESSION_DATA_MISSING_MSG).await?;
    }
    
    Ok(())
}

#[poise::command(slash_command)]
pub(crate) async fn get_time(ctx: Context<'_>, player: Option<User>) -> Output {
    if let Ok(data) = Data::get() {
        let target = player.as_ref().unwrap_or(ctx.author());
        if let Some(end) = data.get_time(target.display_name()) {
            ctx.say(format!("{} a fini en {} !", target.mention(), data.display_elapsed(end))).await?;
        } else {
            ctx.say(format!("{} n'a pas finit.", target.mention())).await?;
        }
    } else {
        ctx.say(SESSION_DATA_MISSING_MSG).await?;
    }

    Ok(())
}