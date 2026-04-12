use std::collections::HashMap;

use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use serenity::all::{GuildChannel, Role, User};

use crate::{commands::session::data::{DiscordData, Status, TeamSetup}, util::{Error, Output, get_json_data, write_json_data}};

const PATH: &'static str = "data/session.json";

#[derive(Serialize, Deserialize)]
pub struct Data {
    status: Status,
    team_setup: TeamSetup,
    pub start_time: NaiveDateTime,
    pub timestamps: HashMap<String, Vec<String>>,
    finished: HashMap<String, NaiveDateTime>
} impl Data {
    fn new(team_size: Option<usize>) -> Self {
        let (status, team_size) = team_size.map_or((Status::Active, 0), |x| (Status::Building, x));

        Self { 
            status,
            team_setup: TeamSetup::new(team_size),
            start_time: Utc::now().naive_utc(),
            timestamps: HashMap::new(),
            finished: HashMap::new()
        }
    }
    pub fn activate(&mut self, roles: Vec<Role>, channels: Vec<GuildChannel>) -> Output {
        self.team_setup.roles = roles.into_iter().map(|x| DiscordData::from(x)).collect();
        self.team_setup.channels = channels.into_iter().map(|x| DiscordData::from(x)).collect();

        self.status = Status::Active;
        self.start_time = Utc::now().naive_utc();
        self.write()
    }
    pub fn close(&mut self) -> Output {
        self.status = Status::Closed;
        self.write()
    }

    pub fn status(&self) -> &Status { &self.status }
    pub fn is_closed(&self) -> bool { 
        match self.status {
            Status::Closed | Status::Pushed => true,
            _ => false
        } 
    }
    pub fn is_building(&self) -> bool {
        if let Status::Building = self.status { true } else { false }
    }
    pub fn is_active(&self) -> bool {
        if let Status::Active = self.status { true } else { false } 
    }

    pub fn get() -> Result<Self, Error> { get_json_data(PATH) }

    fn write(&self) -> Output { write_json_data(self, PATH) }
    pub fn write_new(team_size: Option<usize>) -> Output {
        let x = Self::new(team_size);
        x.write()
    }

    pub fn add_game(&mut self, game: String) -> Result<bool, Error> {
        if self.team_setup.add_game(game) {
            self.write().map(|()| true)
        } else { Ok(false) }
    }
    pub fn add_player(&mut self, player: User) -> Result<(usize, Option<String>), Error> {
        let res = self.team_setup.add_player(player);
        self.write()?;
        Ok(res)
    }

    fn handle_opt<T, F: FnOnce(&mut Self)->Option<T>>(&mut self, mutation: F) -> Result<Option<T>, Error> {
        let res = mutation(self);
        if res.is_some() {
            self.write()?;
        }

        Ok(res)
    }
    pub fn remove_last_game(&mut self) -> Result<Option<String>, Error> {
        self.handle_opt(|x| x.team_setup.remove_last_game())
    }
    pub fn remove_last_player(&mut self) -> Result<Option<(usize, String)>, Error> {
        self.handle_opt(|x| x.team_setup.remove_last_player())
    }

    pub fn roles(&self) -> &[DiscordData] { self.team_setup.roles.as_slice() }
    pub fn channels(&self) -> &[DiscordData] { self.team_setup.channels.as_slice() }

    pub fn n_teams(&self) -> usize { self.team_setup.n_teams() }
    pub fn team_ids(&self) -> impl Iterator<Item = impl Iterator<Item = u64>> { self.team_setup.team_ids() }
    pub fn check_setup(&self) -> bool { self.team_setup.check() }
    pub fn setup_to_string(&self) ->  String { self.team_setup.to_string() } 
    
    pub fn finish(&mut self, player: &str, force: bool) -> Result<Option<NaiveDateTime>, Error> {
        if !force && self.finished.contains_key(player) {   
            return Ok(None); 
        }

        let end = Utc::now().naive_utc();
        self.finished.insert(player.to_string(), end);

        self.write()?;
        return Ok(Some(end));
    }

    pub fn display_elapsed(&self, t: &NaiveDateTime)  -> String {
        let duration = t.signed_duration_since(self.start_time);
        let hours = duration.num_hours();
        let minutes = duration.num_minutes() - 60 * hours;
        let secs = duration.num_seconds() - 3600 * hours - 60 * minutes;

        format!("{:02}:{:02}:{:02}", hours, minutes, secs)
    }

    pub fn get_time(&self, key: &str) -> Option<&NaiveDateTime> {
        self.finished.get(key)
    }

    pub fn add_tms(&mut self, player: &str, label: &str) -> Output {
        let entry = format!("{label}: {}", self.display_elapsed(&Utc::now().naive_utc()));

        if let Some(entries) = self.timestamps.get_mut(player) {
            entries.push(entry);
        } else {
            self.timestamps.insert(player.to_string(), vec![entry]);
        }

        self.write()
    }

    pub fn get_tms(&self, player: &str) -> Option<&Vec<String>> {
        self.timestamps.get(player)
    }
}