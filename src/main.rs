use serde::{Deserialize, Serialize};

use crate::stat::Scores;

mod data;
mod model;
mod stat;
mod test;

fn main() {
    let scores = data::parse_scores_dir().unwrap();

    println!(
        "{}",
        scores.iter().max_run_time().num_seconds() as f64 / 3600.
    );
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    scores_path: String,
}
