use chrono::{Datelike, NaiveDate};
use serde::{
    de::{self},
    Deserialize, Deserializer, Serializer,
};

pub fn de_naivedate<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;

    let year = format!(
        "20{}",
        s.get(0..2).ok_or(de::Error::custom(
            "Could not access first 2 chars of date (year). The string should be exactly 6 chars long."
        ))?
    )
    .parse::<i32>()
    .map_err(|e| de::Error::custom(format!("Error parsing year into integer: {e}")))?;
    let month = s
        .get(2..4)
        .ok_or(de::Error::custom(
            "Could not access chars 3 and 4 of date (month). The string should be exactly 6 chars long.",
        ))?
        .parse::<u32>()
        .map_err(|e| de::Error::custom(format!("Error parsing month into integer: {e}")))?;
    let day = s
        .get(4..6)
        .ok_or(de::Error::custom(
            "Could not access chars 5 and 6 of date (day). The string should be exactly 6 chars long.",
        ))?
        .parse::<u32>()
        .map_err(|e| de::Error::custom(format!("Error parsing day into integer: {e}")))?;

    Ok(
        NaiveDate::from_ymd_opt(year, month, day).ok_or(de::Error::custom(
            "Could not convert year, month, and day into NaiveDate",
        ))?,
    )
}

pub fn ser_naivedate<S>(x: &NaiveDate, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let year = &x.year().to_string()[2..4];
    let month = (x.month0() + 1).to_string();
    let day = (x.day0() + 1).to_string();
    s.serialize_str(&format!("{:0>2}{:0>2}{:0>2}", year, month, day))
}
