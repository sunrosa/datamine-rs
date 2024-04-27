use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct Score {
    pub header: Header,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct Header {
    pub filename: String,
    pub version: String,
    pub run_end_date: String,
    pub run_end_time: String,
    pub player_name: String,
    pub run_result: String,
    pub win: bool,
}
