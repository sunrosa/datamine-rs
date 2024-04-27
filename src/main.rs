use serde::Serialize;

mod model;

fn main() {
    let test_score = std::fs::OpenOptions::new()
        .read(true)
        .open("test.json")
        .unwrap();

    let score: model::Score = serde_json::from_reader(test_score).unwrap();
    println!("{}", serde_json::to_string(&score).unwrap());
}
