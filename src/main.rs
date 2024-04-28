use serde::{Deserialize, Serialize};

mod data;
mod model;
mod stat;
mod test;

fn main() {
    println!(
        "{}",
        serde_json::to_string(&data::parse_scores_dir().unwrap().first().unwrap().header)
            .unwrap()
    );
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    scores_path: String,
}
