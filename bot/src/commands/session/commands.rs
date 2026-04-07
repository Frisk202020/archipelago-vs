use chrono::Utc;
use serenity::all::{Mentionable, User};

use crate::{
    commands::session::{data::Data, interaction::SessionInteraction}, 
    util::{Context, Output, vec_to_list}
};

const SESSION_DATA_MISSING_MSG: &'static str = "Commence peut-être par lancer une session hein.";
const GIF: &'static str = "https://giphy.com/gifs/football-lost-hashtagunited-jU9OCvBiO1besabUKU";

/// Lancer le chrono pour un nouveau versus.
/// 
/// Cela supprime les données relatives à une précédente session.
#[poise::command(slash_command)]
pub(crate) async fn start_session(ctx: Context<'_>) -> Output {
    if Data::get().is_err() {
        Data::write_new()?;
        ctx.say("C'est tout bon").await?;
        return Ok(());
    };

    let res = SessionInteraction::handle_interaction(
        ctx,
        "Pour info une session est en cours selon mes données, et procéder va tout foutre à la poubelle. T'es sur de ton coup ?",
        vec![SessionInteraction::StartAccept, SessionInteraction::StartDeny]
    ).await?;

    if let Some(res) = res {
        match res {
            SessionInteraction::StartAccept => {
                Data::write_new()?;
                ctx.say("Hop là c'est parti").await?;
            },
            _ => {},
        }
    } else {
        ctx.say("Hésite pas à répondre la prochaine fois connard").await?;
    }

    Ok(())
}

/// Marquer la victoire d'un joueur (sauvegarder le temps de jeu actuel).
#[poise::command(slash_command)]
pub(crate) async fn finish(
    ctx: Context<'_>, 
    #[description = "Le joueur qui a terminé, par défaut le client de la commande"] player: Option<User>) -> Output {
    if let Ok(mut data) = Data::get() { 
        let user = player.as_ref().unwrap_or(ctx.author());
        
        if let Some(end) = data.finish(&user.name, false)? {
            ctx.say(format!("GG {}, tu as fini en {} !", user.mention(), data.display_elapsed(&end))).await?;
        } else {
            let res = SessionInteraction::handle_interaction(
                ctx, 
                &format!("Mais... {} avait déjà finit, on remplace par ce nouveau temps ?", user.mention()),
                vec![SessionInteraction::FinishAccept, SessionInteraction::FinishDeny]
            ).await?;

            if let Some(res) = res {
                match res {
                    SessionInteraction::FinishAccept => {
                        let end = data.finish(&user.name, true)?.unwrap();
                        ctx.say(format!("GG {}, tu as fini en {} !", user.mention(), data.display_elapsed(&end))).await?;
                    },
                    _ => {},
                }
            } else {
                ctx.say("Hésite pas à répondre la prochaine fois connard").await?;
            }
        }
    } else {
        ctx.say(SESSION_DATA_MISSING_MSG).await?;
    }
    
    Ok(())
}

/// Afficher le temps de partie enregistré pour un joueur.
#[poise::command(slash_command)]
pub(crate) async fn get_time(
    ctx: Context<'_>, 
    #[description = "Le joueur qui a terminé, par défaut le client de la commande"] player: Option<User>
) -> Output {
    if let Ok(data) = Data::get() {
        let target = player.as_ref().unwrap_or(ctx.author());
        if let Some(end) = data.get_time(&target.name) {
            ctx.say(format!("{} a fini en {} !", target.mention(), data.display_elapsed(end))).await?;
        } else {
            ctx.say(format!("{} n'a pas finit.", target.mention())).await?;
        }
    } else {
        ctx.say(SESSION_DATA_MISSING_MSG).await?;
    }

    Ok(())
}

/// Ajouter un split sur la run d'un joueur.
#[poise::command(slash_command)]
pub(crate) async fn add_tms(
    ctx: Context<'_>, 
    #[description = "Description du split"] label: String, 
    #[description = "Le joueur qui a terminé, par défaut le client de la commande"] player: Option<User>
) -> Output {
    if let Ok(mut data) = Data::get() {
        let name = player.as_ref().map(|x| &x.name).unwrap_or(&ctx.author().name);
        data.add_tms(name.as_str(), label.as_str())?;
        ctx.say("C'est noté.").await?;
    } else {
        ctx.say(SESSION_DATA_MISSING_MSG).await?;
    }

    Ok(())
}

/// Afficher la liste de tous les splits enregistrés.
#[poise::command(slash_command)]
pub(crate) async fn list_tms(
    ctx: Context<'_>, 
    #[description = "Le joueur qui a terminé, par défaut le client de la commande"] player: Option<User>
) -> Output {
    let res = if let Ok(data) = Data::get() {
        let user = player.as_ref().unwrap_or(ctx.author());
        if let Some(tms) = data.get_tms(&user.name) {
            &vec_to_list(tms)
        } else {
            GIF
        }
    } else {
        SESSION_DATA_MISSING_MSG
    };

    ctx.say(res).await?;
    Ok(())
}

/// Afficher la durée de la session en cours.
#[poise::command(slash_command)]
pub(crate) async fn session_duration(ctx: Context<'_>) -> Output {
    let res = if let Ok(data) = Data::get() {
        &data.display_elapsed(&Utc::now().naive_utc())
    } else { SESSION_DATA_MISSING_MSG };

    ctx.say(res).await?;
    Ok(())
}