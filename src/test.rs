#![cfg(test)]

use super::model;

macro_rules! test_number {
    ($fn_name: ident, $num:literal) => {
        #[test]
        fn $fn_name() {
            des_and_ser($num);
        }
    };
}

fn des_and_ser(filename: &str) {
    let test_score = std::fs::OpenOptions::new()
        .read(true)
        .open(format!("test_data/{}.json", filename))
        .expect("Error reading test score from file");

    let score: model::Score =
        serde_json::from_reader(test_score).expect("Error deserializing test score");

    println!(
        "{}",
        serde_json::to_string(&score).expect("Error serializing test score")
    );
}

test_number!(ds0, "0");
test_number!(ds1, "1");
test_number!(ds2, "2");
test_number!(ds3, "3");
test_number!(ds4, "4");
test_number!(ds5, "5");
test_number!(ds6, "6");
test_number!(ds7, "7");
test_number!(ds8, "8");
test_number!(ds9, "9");
test_number!(ds10, "10");
test_number!(ds11, "11");
test_number!(ds12, "12");
test_number!(ds13, "13");
test_number!(ds14, "14");
