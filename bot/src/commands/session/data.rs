use std::collections::HashMap;

use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::util::{Error, Output, get_json_data, write_json_data};

const PATH: &'static str = "data/session.json";

#[derive(Serialize, Deserialize)]
pub(crate) struct Data {
    pub(crate) start_time: NaiveDateTime,
    pub(crate) timestamps: HashMap<String, Vec<String>>,
    finished: HashMap<String, NaiveDateTime>
} impl Data {
    pub(crate) fn get() -> Result<Self, Error> { get_json_data(PATH) }
    fn write(&self) -> Output { write_json_data(self, PATH) }

    fn default() -> Self {
        Self { 
            start_time: Utc::now().naive_utc(),
            timestamps: HashMap::new(),
            finished: HashMap::new()
        }
    }

    pub(crate) fn write_new() -> Output {
        let x = Self::default();
        x.write()
    }
    
    pub(crate) fn finish(&mut self, player: &str, force: bool) -> Result<Option<NaiveDateTime>, Error> {
        if !force && self.finished.contains_key(player) {   
            return Ok(None); 
        }

        let end = Utc::now().naive_utc();
        self.finished.insert(player.to_string(), end);

        self.write()?;
        return Ok(Some(end));
    }

    pub(crate) fn display_elapsed(&self, t: &NaiveDateTime)  -> String {
        let duration = t.signed_duration_since(self.start_time);
        let hours = duration.num_hours();
        let minutes = duration.num_minutes() - 60 * hours;
        let secs = duration.num_seconds() - 3600 * hours - 60 * minutes;

        format!("{:02}:{:02}:{:02}", hours, minutes, secs)
    }

    pub(crate) fn get_time(&self, key: &str) -> Option<&NaiveDateTime> {
        self.finished.get(key)
    }

    pub(crate) fn add_tms(&mut self, player: &str, label: &str) -> Output {
        let entry = format!("{label}: {}", self.display_elapsed(&Utc::now().naive_utc()));
        println!("{}", Utc::now().naive_utc());

        if let Some(entries) = self.timestamps.get_mut(player) {
            entries.push(entry);
        } else {
            self.timestamps.insert(player.to_string(), vec![entry]);
        }

        self.write()
    }

    pub(crate) fn get_tms(&self, player: &str) -> Option<&Vec<String>> {
        self.timestamps.get(player)
    }
}