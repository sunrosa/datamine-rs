use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct Score {
    pub header: Header,
    pub performance: Performance,
    pub bonus: Bonus,
    pub cogmind: Cogmind,
    pub parts: Parts,
    pub peak_state: PeakState,
    pub favorites: Favorites,
    pub class_distribution: ClassDistribution,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub history_event_win: Option<HistoryEventWin>,

    pub last_messages: LastMessages,
    pub map: Map,
    pub best_states: BestStates,
    pub alien_tech_used: AlienTechUsed,
    pub achievements: Achievements,
    pub challenges: Challenges,
    pub cogshop_purchases: CogshopPurchases,
    pub game: Game,
    pub options: Options,
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct PeakState {
    #[serde(flatten)]
    parts: Parts,
    rating: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct Favorites {
    power: PowerFavorites,
    propulsion: PropulsionFavorites,
    utility: UtilityFavorites,
    weapon: WeaponFavorites,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct PowerFavorites {
    overall: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    engine: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    power_core: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    reactor: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct PropulsionFavorites {
    overall: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    leg: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    hover_unit: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    flight_unit: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    treads: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    wheel: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct UtilityFavorites {
    overall: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    device: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    storage: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    processor: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    hackware: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    protection: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct WeaponFavorites {
    overall: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    energy_gun: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    energy_cannon: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    ballistic_cannon: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    ballistic_gun: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    launcher: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    special_weapon: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    impact_weapon: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct ClassDistribution {
    classes: Vec<ClassPercent>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct ClassPercent {
    name: String,
    percent: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct HistoryEventWin {
    turn: i32,
    event: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct LastMessages {
    messages: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct Map {
    lines: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct BestStates {
    #[serde(skip_serializing_if = "Option::is_none")]
    heat_dissipation: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    energy_generation: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    energy_capacity: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    matter_stores: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    matter_capacity: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sight_range: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    terrain_scan_density: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    evasion: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    offensive_hacking: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    defensive_hacking: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    additional_mass_support: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    power_amplification: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    jamming_range: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    coolant_potential: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    robot_scan_range: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    terrain_scan_range: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    ecm_strength: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    cloak_strength: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    em_shielding: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    armor_coverage: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    resistance_ki: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    resistance_th: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    recoil_reduction: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    utility_shielding: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    targeting_accuracy: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    weapon_shielding: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    targeting_analysis: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    resistance_em: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    target_analysis: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    matter_filtering: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    phase_shifting: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    core_analysis: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    particle_charging: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct AlienTechUsed {
    #[serde(skip_serializing_if = "Option::is_none")]
    parts: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct Achievements {
    #[serde(skip_serializing_if = "Option::is_none")]
    achievements: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct Challenges {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct CogshopPurchases {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct Game {
    world_seed: String,
    run_time: String,
    cumulative_hours: String,
    run_start_date: String,
    run_end_date: String,
    run_sessions: i32,
    game_number: i32,
    game_counts: Vec<i32>,
    win_type: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    win_total: Option<i32>,
    win_type_history: Vec<i32>,
    lore_percent: i32,
    gallery_percent: i32,
    achievement_percent: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct Options {
    keyboard: bool,
    movement: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    keybinds: Option<bool>,
    fullscreen: String,
    font_set: String,
    map_width: i32,
    map_height: i32,
    tactical_hud: bool,
    render_filters_map: String,
    render_filters: String,
    steam: String,
}
