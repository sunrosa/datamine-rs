use serde::{Deserialize, Serialize};

mod data;
mod model;
mod stat;
mod test;

fn main() {
    println!(
        "{:?}\n\n{}",
        data::parse_scores_dir().unwrap().first().unwrap().game,
        serde_json::to_string(&data::parse_scores_dir().unwrap().first().unwrap().game)
            .unwrap()
    );
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    scores_path: String,
}
