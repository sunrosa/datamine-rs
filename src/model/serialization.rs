use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Deserializer, Serializer};

//TODO Error processing instead of unwrap
pub fn into_naivedate<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;

    let year = format!("20{}", &s[0..2]).parse::<i32>().unwrap();
    let month = s[2..4].parse::<u32>().unwrap();
    let day = s[4..6].parse::<u32>().unwrap();

    Ok(NaiveDate::from_ymd_opt(year, month, day).unwrap())
}

pub fn from_naivedate<S>(x: &NaiveDate, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let year = &x.year().to_string()[2..4];
    let month = (x.month0() + 1).to_string();
    let day = (x.day0() + 1).to_string();
    s.serialize_str(&format!("{:0>2}{:0>2}{:0>2}", year, month, day))
}
