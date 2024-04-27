mod model;

fn main() {
    let test_score = std::fs::OpenOptions::new()
        .read(true)
        .open("test.json")
        .expect("Error reading test score from file");

    let score: model::Score =
        serde_json::from_reader(test_score).expect("Error deserializing test score");

    println!(
        "{}",
        serde_json::to_string(&score).expect("Error serializing test score")
    );
}
