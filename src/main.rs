use serde::{Deserialize, Serialize};
use stat::Scores;

mod data;
mod model;
mod stat;
mod test;

fn main() {
    println!(
        "{:?}",
        data::parse_scores_dir().unwrap().into_iter().win_count()
    );
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    scores_path: String,
}
