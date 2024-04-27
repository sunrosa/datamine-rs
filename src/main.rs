mod model;
mod test;

fn main() {
    let test_score = std::fs::OpenOptions::new()
        .read(true)
        .open("test_data/0.json")
        .expect("Error reading test score from file");

    let score: model::Score =
        serde_json::from_reader(test_score).expect("Error deserializing test score");

    println!(
        "{:?}",
        score
            .route
            .entries
            .get(1)
            .unwrap()
            .stats
            .combat
            .damage_inflicted
    );
}
