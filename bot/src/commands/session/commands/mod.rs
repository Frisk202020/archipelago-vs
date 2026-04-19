use serenity::all::{ChannelId, Mentionable, RoleId, User};
use crate::{
    commands::session::{
        data::Data, 
        interaction::{NO_REPLY, SessionInteraction},
        commands::{builder::builder, timestamp::timestamps}
    }, 
    util::{Context, Output},
};

mod builder;
mod timestamp;

const SESSION_DATA_MISSING_MSG: &'static str = "Commence peut-être par lancer une session hein.";

#[poise::command(slash_command, subcommands("builder", "timestamps", "finish", "close", "get_time"))]
pub async fn session(_ctx: Context<'_>) -> Output { Ok(()) }

/// Marquer la fin de partie d'un joueur
/// 
/// Il n'est pas nécessaire que tout le monde finisse pour clôturer la session
#[poise::command(slash_command)]
async fn finish(
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
                ctx.say(NO_REPLY).await?;
            }
        }
    } else {
        ctx.say(SESSION_DATA_MISSING_MSG).await?;
    }
    
    Ok(())
}

/// Afficher le temps de partie enregistré pour un joueur.
#[poise::command(slash_command)]
async fn get_time(
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

/// Clôturer la session en cours
/// 
/// @todo push data to Sheets API
#[poise::command(slash_command)]
async fn close(ctx: Context<'_>) -> Output {
    let mut data = Data::get()?;
    if !data.is_active() {
        ctx.say("Aucune session  active").await?;
        return Ok(())
    }

    if let Some(guild) = ctx.partial_guild().await { 'delete_channels: {
        let channel_data = data.channels();
        if channel_data.is_empty() { break 'delete_channels; }

        let channels = guild.channels(ctx.http()).await?;
        for data in channel_data {
            if let Some(channel) = channels.get(&ChannelId::new(data.id)) {
                if channel.name == data.name {
                    channel.delete(ctx.http()).await?;
                } else {
                    ctx.say(format!("Un channel ne sera pas supprimé car son nom ne correspond pas à mes données : {}", channel.name)).await?;
                }
            } else {
                ctx.say(format!("Un channel dans ma base n'a pas été trouvé : {}", data.name)).await?;
            }
        }

        let roles = data.roles();
        for role in roles {
            guild.delete_role(ctx.http(), RoleId::new(role.id)).await?;
        }

        data.close()?;
        ctx.say("Session clôturée avec succès !").await?;
    }} else {
        ctx.say("Je ne trouve pas le server").await?;
    } 
    Ok(())
}