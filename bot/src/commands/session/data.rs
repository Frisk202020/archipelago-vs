use std::{collections::HashMap, time::Instant};

use serde::{Deserialize, Serialize};

use crate::util::{Error, get_json_data, write_json_data};

const PATH: &'static str = "data/session.json";

#[derive(Serialize, Deserialize)]
pub(crate) struct Data {
    pub(crate) active: bool,
    pub(crate) start_time: u64,
    pub(crate) finished: HashMap<String, u64>
} impl Data {
    pub(crate) fn get() -> Result<Self, Error> { get_json_data(PATH) }
    pub(crate) fn write(&self) -> Result<(), Error> { write_json_data(self, PATH) }
} impl Default for Data {
    fn default() -> Self {
        Self { 
            active: true, 
            start_time: Instant::now().elapsed().as_secs(),
            finished: HashMap::new()
        }
    }
}