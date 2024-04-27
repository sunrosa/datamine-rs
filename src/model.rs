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
    pub route: Route,
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
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct Stats {
    pub build: Build,
    pub resources: Resources,
    pub kills: Kills,
    pub combat: Combat,
    pub alert: Alert,
    pub stealth: Stealth,
    pub traps: Traps,
    pub machines: Machines,
    pub hacking: Hacking,
    pub bothacking: Bothacking,
    pub allies: Allies,
    pub intel: Intel,
    pub exploration: Exploration,
    pub actions: Actions,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_weapon: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reactor: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct HeaviestBuild {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    pub greatest_support: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub greatest_overweight_times: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_overweight_times: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct LargestInventoryCapacity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_capacity: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub most_carried: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_carried: Option<i32>,
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct Resources {
    pub matter_collected: MatterCollected,
    pub salvage_created: SalvageCreated,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub haulers_intercepted: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recyclers_shooed: Option<i32>,

    pub parts_field_recycled: PartsFieldRecycled,
    pub parts_self_destructed: PartsSelfDestructed,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct MatterCollected {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct SalvageCreated {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parts: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct PartsFieldRecycled {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_matter: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct PartsSelfDestructed {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prevented: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct Kills {
    pub combat_hostiles_destroyed: CombatHostilesDestroyed,
    pub classes_destroyed: ClassesDestroyed,
    pub best_kill_streak: BestKillStreak,
    pub max_kills_in_single_turn: MaxKillsInSingleTurn,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct CombatHostilesDestroyed {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub guns: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cannons: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub melee: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub aoe: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct ClassesDestroyed {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub builder: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tunneler: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hauler: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recycler: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub watcher: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub swarmer: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub grunt: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub brawler: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentry: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub programmer: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duelist: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub drone: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mechanic: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub saboteur: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub assembled: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hunter: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protector: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub behemoth: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub demolisher: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub specialist: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thug: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutant: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct BestKillStreak {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub combat_bots_only: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct MaxKillsInSingleTurn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exploded: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gunslinging: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub melee: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct Combat {
    pub damage_taken: DamageTaken,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_remaining_percent: Option<i32>,

    pub volleys_fired: VolleysFired,
    pub shots_fired: ShotsFired,
    pub shots_hit_robots: ShotsHitRobots,
    pub melee_attacks: MeleeAttacks,
    pub damage_inflicted: DamageInflicted,
    pub highest_corruption: HighestCorruption,
    pub overload_shots: OverloadShots,
    pub overflow_damage: OverflowDamage,
    pub knockbacks: Knockbacks,
    pub self_inflicted_damage: SelfInflictedDamage,
    pub targets_rammed: TargetsRammed,
    pub highest_temperature: HighestTemperature,
    pub siege_activations: SiegeActivations,
    pub robots_corrupted: RobotsCorrupted,
    pub robots_melted: RobotsMelted,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_chain_reactions: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parts_sabotaged: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parts_stolen: Option<i32>,

    pub latent_energy_used: LatentEnergyUsed,
    pub hostile_shots_fired: HostileShotsFired,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct DamageTaken {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub core: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub regen_repair_parts: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirected_to_shielding: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub absorbed_by_shields: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignored_by_resistances: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct VolleysFired {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub largest: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hottest: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct ShotsFired {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gun: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cannon: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub launcher: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub special: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinetic: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thermal: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub explosive: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub electromagnetic: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub robot_hit_streak: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub robot_miss_streak: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub penetration_max: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub impact: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub autonomous: Option<i32>,

    pub secondary_targets: SecondaryTargets,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct SecondaryTargets {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_gunslinging_chain: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct ShotsHitRobots {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_hits: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub critical_kills: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub critical_parts_destroyed: Option<i32>,

    pub critical_strikes: CriticalStrikes,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct CriticalStrikes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub burn: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub destroy: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub blast: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub meltdown: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub corrupt: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct MeleeAttacks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub impact: Option<i32>,

    pub sneak_attacks: SneakAttacks,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct SneakAttacks {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct DamageInflicted {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub guns: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cannons: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub explosions: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ramming: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinetic: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thermal: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub explosive: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub electromagnetic: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub impact: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub melee: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct HighestCorruption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_corruption: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub corruption_purged: Option<i32>,

    pub effects: HighestCorruptionEffects,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct HighestCorruptionEffects {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_errors: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub matter_fused: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub heat_flow_errors: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_discharges: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_loss_minor: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_loss_major: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub alerts: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parts_fused: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parts_rejected: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub misfires: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub misdirections: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub weapon_failures: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct OverloadShots {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    pub effects: OverloadShotsEffects,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct OverloadShotsEffects {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_bleed: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub heat_surge: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct OverflowDamage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub projectiles: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub explosions: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub melee: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct Knockbacks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub impact: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinetic: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct SelfInflictedDamage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shots: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rammed: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct TargetsRammed {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub kicked: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub crushed: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct HighestTemperature {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_temperature: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_heat_transfer: Option<i32>,

    pub effects: HighestTemperatureEffects,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct HighestTemperatureEffects {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shutdowns: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_bleed: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub matter_decay: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interference: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub damage_minor: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub damage_major: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub damage_core: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_circuit: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct SiegeActivations {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct RobotsCorrupted {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parts_fried: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub combat_hostiles: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub impact_corruptions: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct RobotsMelted {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub combat_hostiles: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parts_melted: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub heat_transferred: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct LatentEnergyUsed {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub le_corruption: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct HostileShotsFired {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub missed: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct Alert {
    pub peak_influence: PeakInfluence,
    pub maximum_alert_level: MaximumAlertLevel,
    pub squads_dispatched: SquadsDispatched,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub construction_impeded: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub haulers_reinforced: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cargo_convoy_interrupts: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub searches_triggered: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct PeakInfluence {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_influence: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_influence: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct MaximumAlertLevel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_security_percent: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub level1: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub level2: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub level3: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub level4: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub level5: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_security: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct SquadsDispatched {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub investigation: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extermination: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reinforcement: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub assault: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub garrison: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct Stealth {
    #[serde(skip_serializing_if = "Option::is_none")]
    distress_signals: Option<i32>,

    communications_jammed: CommunicationsJammed,

    times_spotted: TimesSpotted,

    id_masks_used: IdMasksUsed,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct CommunicationsJammed {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub distress_signals: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct TimesSpotted {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub peak_tracking_total: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tactical_retreats: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct IdMasksUsed {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct Traps {
    pub traps_triggered: TrapsTriggered,
    pub trap_hack_attempts: TrapHackAttempts,
    pub traps_extracted: TrapsExtracted,
    pub objects_rigged: ObjectsRigged,
    pub time_bombs_activated: TimeBombsActivated,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct TrapsTriggered {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub indirectly: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct TrapHackAttempts {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disarmed: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct TrapsExtracted {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggered: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct ObjectsRigged {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct TimeBombsActivated {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct Machines {
    pub machines_disabled: MachinesDisabled,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct MachinesDisabled {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_in_single_turn: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub garrison_access: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct Hacking {
    pub machines_accessed: MachinesAccessed,
    pub total_hacks: TotalHacks,
    pub terminal_hacks: TerminalHacks,
    pub fabricator_hacks: FabricatorHacks,
    pub repair_station_hacks: RepairStationHacks,
    pub recycling_unit_hacks: RecyclingUnitHacks,
    pub scanalyzer: ScanalyzerHacks,
    pub garrison_access_hacks: GarrisonAccessHacks,
    pub unauthorized_hacks: UnauthorizedHacks,
    pub data_cores_recovered: DataCoresRecovered,
    pub hacking_detections: HackingDetections,
    pub robot_schematics_acquired: RobotSchematicsAcquired,
    pub part_schematics_acquired: PartSchematicsAcquired,
    pub parts_repaired: PartsRepaired,
    pub parts_recycled: PartsRecycled,
    pub parts_scanalyzed: PartsScanalyzed,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct MachinesAccessed {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminals: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fabricators: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub repair_stations: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recycling_units: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanalyzers: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub garrison_access: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct TotalHacks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful: Option<i32>,

    pub failed: FailedHacks,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_lockouts: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminals: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fabricators: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub repair_stations: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recycling_units: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanalyzers: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub garrison_access: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct FailedHacks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub catastrophic: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct TerminalHacks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub record: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_schematic: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub robot_schematic: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub robot_analysis: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prototype_id_bank: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_door: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminal_index: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_level: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub patrol_status: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_components: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_dsf: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unreport_threat: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_access_points: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_access_points: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub emergency_access_points: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fabricator_index: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recycling_unit_index: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub repair_station_index: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub locate_traps: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recall_reinforcements: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recall_investigation: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_layout: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_status: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_status: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disarm_traps: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hauler_manifests: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct FabricatorHacks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_status: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_schematic: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct RepairStationHacks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_component: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub repair: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct RecyclingUnitHacks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub process: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_inventory: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieve_components: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieve_matter: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct ScanalyzerHacks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_component: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyze: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct GarrisonAccessHacks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub seal_access: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlock_access: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupler_status: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct UnauthorizedHacks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    pub terminals: TerminalUnauthorizedHacks,
    pub fabricators: FabricatorUnauthorizedHacks,
    pub repair_stations: RepairStationUnauthorizedHacks,
    pub recycling_units: RecyclingUnitUnauthorizedHacks,
    pub scanalyzers: ScanalyzerUnauthorizedHacks,
    pub garrison_access: GarrisonAccessUnauthorizedHacks,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct TerminalUnauthorizedHacks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub botnet: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub operators: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub assimilate: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct FabricatorUnauthorizedHacks {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct RepairStationUnauthorizedHacks {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct RecyclingUnitUnauthorizedHacks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tunnel: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct ScanalyzerUnauthorizedHacks {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct GarrisonAccessUnauthorizedHacks {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct DataCoresRecovered {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub used: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct HackingDetections {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_trace_events: Option<i32>,

    pub feedback_events: FeedbackEvents,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct FeedbackEvents {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct RobotSchematicsAcquired {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    pub robots_built: RobotsBuilt,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct RobotsBuilt {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct PartSchematicsAcquired {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    pub parts_built: PartsBuilt,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_part_build_rating: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct PartsBuilt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct PartsRepaired {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct PartsRecycled {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_components: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct PartsScanalyzed {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_schematics_acquired: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parts_damaged: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct Bothacking {
    used_rif_installer: UsedRifInstaller,
    robots_hacked: RobotsHacked,
    robot_hacks_applied: RobotHacksApplied,
    relay_couplers_released: RelayCouplersReleased,
    fabnet_peak_effective_percent: FabnetPeakEffectivePercent,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub robots_rewired: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct UsedRifInstaller {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct RobotsHacked {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_combat: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct RobotHacksApplied {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_system: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_distress: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_alert: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub find_chute: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct RelayCouplersReleased {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub machine_destruction: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub programmers: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct FabnetPeakEffectivePercent {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct Allies {
    pub total_allies: TotalAllies,
    pub zionite_dispatches: ZioniteDispatches,
    pub total_orders: TotalOrders,
    pub ally_attacks: AllyAttacks,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allies_corrupted: Option<i32>,

    pub warlord_squad_rendezvous: WarlordSquadRendezvous,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct TotalAllies {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub largest_group: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub highest_rated_group: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub highest_rated_ally: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct ZioniteDispatches {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub heavy: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub light: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct TotalOrders {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub explore: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stay: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub follow: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct AllyAttacks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_damage: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub kills: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct WarlordSquadRendezvous {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct Intel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robot_analysis_total: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub derelict_logs_recovered: Option<i32>,

    pub drone_launches: DroneLaunches,
    pub decoded_0b10_intel: Decoded0b10Intel,
    pub zionite_intel_received: ZioniteIntelReceived,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct DroneLaunches {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub drone_recoveries: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct Decoded0b10Intel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub traps: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub emergency_access: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub machines: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub patrols: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub guards: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct ZioniteIntelReceived {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prototype_stockpiles: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prototype_schematics: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unaware_analyses: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct Exploration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turns_passed: Option<i32>,

    pub spaces_moved: SpacesMoved,
    pub exploration_rate_percent: ExplorationRatePercent,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scrap_searched: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub spaces_dug: Option<i32>,

    pub terrain_destroyed: TerrainDestroyed,

    pub terrain_rammed: TerrainRammed,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct SpacesMoved {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub core: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub treads: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wheels: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub legs: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub flight: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fastest_speed: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_speed: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub slowest_speed: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub robots_hopped: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_travels: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    overloaded_moves: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    cave_ins_triggered: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct ExplorationRatePercent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    pub regions_visited: RegionsVisited,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_discovered_areas: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub known_exits_taken: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_exits_taken: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct RegionsVisited {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_regions: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct TerrainDestroyed {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub projectile: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub aoe: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct TerrainRammed {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct Actions {
    pub total: TotalActions,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct TotalActions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "move")]
    pub moves: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pick_up: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fast_attach: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub detach: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fire: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ram: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub crush: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub escape_stasis: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rewire: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub melee: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hop: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub kick: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub miscellaneous: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct Route {
    pub entries: Vec<RouteEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct RouteEntry {
    pub location: Location,
    pub discovered_exits: Vec<DiscoveredExit>,
    pub stats: Stats,
    pub history_events: Vec<HistoryEvent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub obtained_schematics: Option<Vec<Schematic>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dominant_class: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fabricated_objects: Option<Vec<FabricatedObject>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct DiscoveredExit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_known: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reached: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct HistoryEvent {
    pub turn: i32,
    pub event: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct Schematic {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub schematic_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    rename_all(serialize = "camelCase", deserialize = "camelCase"),
    deny_unknown_fields
)]
pub struct FabricatedObject {
    name: String,
    count: i32,
}
