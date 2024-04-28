use serde::{Deserialize, Serialize};

use crate::stat::ScoresRef;

mod data;
mod model;
mod stat;
mod test;

fn main() {
    let scores = data::parse_scores_dir().unwrap();

    println!("{}", scores.iter().total_score());
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    scores_path: String,
}
