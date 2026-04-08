use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) enum Status {
    Building,
    Active,
    Closed,
    Pushed
}