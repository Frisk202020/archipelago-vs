use std::collections::HashMap;

use chrono::{NaiveTime, Utc};
use serde::{Deserialize, Serialize};

use crate::util::{Error, get_json_data, write_json_data};

const PATH: &'static str = "data/session.json";

#[derive(Serialize, Deserialize)]
pub(crate) struct Data {
    pub(crate) active: bool,
    pub(crate) start_time: NaiveTime,
    pub(crate) timestamps: HashMap<String, NaiveTime>,
    finished: HashMap<String, NaiveTime>
} impl Data {
    pub(crate) fn get() -> Result<Self, Error> { get_json_data(PATH) }
    pub(crate) fn write(&self) -> Result<(), Error> { write_json_data(self, PATH) }
    pub(crate) fn finish(&mut self, player: &str, force: bool) -> Option<NaiveTime> {
        if !force && self.finished.contains_key(player) {   
            return None; 
        }

        let end = Utc::now().time();
        self.finished.insert(player.to_string(), end);
        return Some(end);
    }

    pub(crate) fn display_elapsed(&self, t: NaiveTime)  -> String {
        let duration = t.signed_duration_since(self.start_time);
        let hours = duration.num_hours();
        let minutes = duration.num_minutes() - 60 * hours;
        let secs = duration.num_seconds() - 3600 * hours - 60 * minutes;

        format!("{hours}:{minutes}:{secs}")
    }
} impl Default for Data {
    fn default() -> Self {
        Self { 
            active: true, 
            start_time: Utc::now().time(),
            timestamps: HashMap::new(),
            finished: HashMap::new()
        }
    }
}