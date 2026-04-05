use std::fs;

use serde::{Serialize, de::DeserializeOwned};
use serenity::all::UserId;

pub(crate) struct RequestData {}
pub(crate) type Error = Box<dyn std::error::Error + Send + Sync>;
pub(crate) type Output = Result<(), Error>;
pub(crate) type Context<'a> = poise::Context<'a, RequestData, Error>;

pub(crate) const DEVELOPER: UserId = UserId::new(526147484716761098);

pub(crate) fn get_json_data<T: DeserializeOwned>(path: &str) -> Result<T, Error> {
    let raw_data = fs::read_to_string(path)?;
    Ok(serde_json::from_str::<T>(raw_data.as_str())?)
}

pub(crate) fn write_json_data<T: Serialize>(data: &T, path: &str) -> Result<(), Error> {
    let update = serde_json::to_string(data)?;
    fs::write(path, update)?;
    
    Ok(())
}

pub(crate) fn vec_to_list(x: &Vec<String>) -> String {
    x.iter()
        .into_iter()
        .map(|x| format!("- {x}\n"))
        .collect::<String>()
}