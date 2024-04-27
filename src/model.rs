use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct Score {
    pub header: Header,
    pub performance: Performance,
    pub bonus: Bonus,
    pub cogmind: Cogmind,
    pub parts: Parts,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct Header {
    pub filename: String,
    pub version: String,
    pub run_end_date: String,
    pub run_end_time: String,
    pub player_name: String,
    pub run_result: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub win: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct Performance {
    total_score: i32,
    evolutions: CountPoints,
    regions_visited: CountPoints,
    robots_destroyed: CountPoints,
    value_destroyed: CountPoints,
    prototypes_identified: CountPoints,
    alien_tech_used: CountPoints,
    bonus: CountPoints,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct CountPoints {
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    points: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct Bonus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggered_max_security: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_alert_combat_kills: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub follower_combat_kills: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entered_garrisons: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub aligned_with_farcom: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_core_reset_matrix: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub win: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entered_dsfs: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub friendly_fire: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub met_data_miner: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub met_imprinter: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub was_imprinted: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct Cogmind {
    core_integrity: CurrentMaximum,
    matter: CurrentMaximum,
    energy: CurrentMaximum,

    #[serde(skip_serializing_if = "Option::is_none")]
    system_corruption: Option<i32>,

    temperature: Temperature,
    location: Location,
    movement: Movement,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct CurrentMaximum {
    #[serde(skip_serializing_if = "Option::is_none")]
    current: Option<i32>,

    maximum: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct Temperature {
    #[serde(skip_serializing_if = "Option::is_none")]
    heat: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct Location {
    #[serde(skip_serializing_if = "Option::is_none")]
    depth: Option<i32>,

    map: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct Movement {
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<String>,

    speed: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    overweight_factor: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct Parts {
    power: SlotsParts,
    propulsion: SlotsParts,
    utility: SlotsParts,
    weapon: SlotsParts,
    inventory: SlotsParts,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct SlotsParts {
    slots: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    parts: Option<Vec<String>>,
}
