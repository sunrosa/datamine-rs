use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    //TODO deny_unknown_fields
)]
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
    pub meta: Meta,
    pub stats: Stats,
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
    pub total_score: i32,
    pub evolutions: CountPoints,
    pub regions_visited: CountPoints,
    pub robots_destroyed: CountPoints,
    pub value_destroyed: CountPoints,
    pub prototypes_identified: CountPoints,
    pub alien_tech_used: CountPoints,
    pub bonus: CountPoints,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct CountPoints {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub points: Option<i32>,
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
    pub core_integrity: CurrentMaximum,
    pub matter: CurrentMaximum,
    pub energy: CurrentMaximum,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_corruption: Option<i32>,

    pub temperature: Temperature,
    pub location: Location,
    pub movement: Movement,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct CurrentMaximum {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<i32>,

    pub maximum: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct Temperature {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heat: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct Location {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<i32>,

    pub map: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct Movement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,

    pub speed: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub overweight_factor: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct Parts {
    pub power: SlotsParts,
    pub propulsion: SlotsParts,
    pub utility: SlotsParts,
    pub weapon: SlotsParts,
    pub inventory: SlotsParts,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct SlotsParts {
    pub slots: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parts: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct PeakState {
    #[serde(flatten)]
    pub parts: Parts,
    pub rating: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct Favorites {
    pub power: PowerFavorites,
    pub propulsion: PropulsionFavorites,
    pub utility: UtilityFavorites,
    pub weapon: WeaponFavorites,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct PowerFavorites {
    pub overall: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_core: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reactor: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct PropulsionFavorites {
    pub overall: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leg: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover_unit: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub flight_unit: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub treads: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wheel: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct UtilityFavorites {
    pub overall: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hackware: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct WeaponFavorites {
    pub overall: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_gun: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_cannon: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ballistic_cannon: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ballistic_gun: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub launcher: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_weapon: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub impact_weapon: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct ClassDistribution {
    pub classes: Vec<ClassPercent>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct ClassPercent {
    pub name: String,
    pub percent: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct HistoryEventWin {
    pub turn: i32,
    pub event: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct LastMessages {
    pub messages: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct Map {
    pub lines: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct BestStates {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heat_dissipation: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_generation: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_capacity: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub matter_stores: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub matter_capacity: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sight_range: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub terrain_scan_density: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub evasion: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub offensive_hacking: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub defensive_hacking: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_mass_support: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_amplification: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub jamming_range: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub coolant_potential: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub robot_scan_range: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub terrain_scan_range: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecm_strength: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloak_strength: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub em_shielding: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub armor_coverage: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resistance_ki: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resistance_th: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recoil_reduction: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub utility_shielding: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub targeting_accuracy: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub weapon_shielding: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub targeting_analysis: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resistance_em: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_analysis: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub matter_filtering: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase_shifting: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_analysis: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub particle_charging: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct AlienTechUsed {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parts: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct Achievements {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub achievements: Option<Vec<String>>,
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
    pub world_seed: String,
    pub run_time: String,
    pub cumulative_hours: String,
    pub run_start_date: String,
    pub run_end_date: String,
    pub run_sessions: i32,
    pub game_number: i32,
    pub game_counts: Vec<i32>,
    pub win_type: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub win_total: Option<i32>,
    pub win_type_history: Vec<i32>,
    pub lore_percent: i32,
    pub gallery_percent: i32,
    pub achievement_percent: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct Options {
    pub keyboard: bool,
    pub movement: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub keybinds: Option<bool>,
    pub fullscreen: String,
    pub font_set: String,
    pub map_width: i32,
    pub map_height: i32,
    pub tactical_hud: bool,
    pub render_filters_map: String,
    pub render_filters: String,
    pub steam: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct Meta {
    pub run_guid: String,
    pub player_public_key: String,
    pub player_guid: String,
    pub player_id: i32,
    pub run_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    //TODO deny_unknown_fields
)]
pub struct Stats {
    pub build: Build,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct Build {
    pub slots_evolved: SlotsEvolved,
    pub parts_attached: PartsAttached,
    pub parts_lost: PartsLost,
    pub average_slot_usage_percent: AverageSlotUsagePercent,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub naked_turns: Option<i32>,

    pub heaviest_build: HeaviestBuild,
    pub largest_inventory_capacity: LargestInventoryCapacity,
    pub scrap_engine_consumption: ScrapEngineConsumption,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct SlotsEvolved {
    pub overall: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub power: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub propulsion: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub utility: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub weapon: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct PartsAttached {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    pub power: PowerAttached,
    pub propulsion: PropulsionAttached,
    pub utility: UtilityAttached,
    pub weapon: WeaponAttached,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unidentified_prototypes: Option<i32>,

    pub corrupted_parts: CorruptedParts,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct PowerAttached {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub core: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reactor: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct PropulsionAttached {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub leg: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub flight: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub treads: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wheel: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct UtilityAttached {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hackware: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct WeaponAttached {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_gun: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_cannon: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ballistic_gun: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ballistic_cannon: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub launcher: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_weapon: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub impact_weapon: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct CorruptedParts {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_corruption: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct PartsLost {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub power: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub propulsion: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub utility: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub weapon: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub highest_loss_streak: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_critical_strikes: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct AverageSlotUsagePercent {
    pub overall: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub core: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub leg: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_gun: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_cannon: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ballistic_gun: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ballistic_cannon: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub launcher: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub treads: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub flight: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hackware: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wheel: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct HeaviestBuild {
    pub overall: i32,
    pub greatest_support: i32,
    pub greatest_overweight_times: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_overweight_times: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct LargestInventoryCapacity {
    pub overall: i32,
    pub average_capacity: i32,
    pub most_carried: i32,
    pub average_carried: i32,
    pub final_capacity: i32,
    pub final_carried: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct ScrapEngineConsumption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub constructs_created: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub constructs_modified: Option<i32>,
}
