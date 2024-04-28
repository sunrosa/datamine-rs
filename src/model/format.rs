use chrono::{Datelike, Duration, NaiveDate, NaiveTime, Timelike};
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
        NaiveDate::from_ymd_opt(year, month, day).ok_or(de::Error::custom(format!(
            "Could not convert year ({year}), month ({month}), and day ({day}), into NaiveDate"
        )))?,
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

pub fn de_naivetime<'de, D>(deserializer: D) -> Result<NaiveTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;

    let hour =
        s.get(0..2).ok_or(de::Error::custom(
            "Could not access first 2 chars of date (hour). The string should be exactly 6 chars long."
        ))?
    .parse::<u32>()
    .map_err(|e| de::Error::custom(format!("Error parsing year into integer: {e}")))?;
    let minute = s
        .get(2..4)
        .ok_or(de::Error::custom(
            "Could not access chars 3 and 4 of date (minute). The string should be exactly 6 chars long.",
        ))?
        .parse::<u32>()
        .map_err(|e| de::Error::custom(format!("Error parsing month into integer: {e}")))?;
    let second = s
        .get(4..6)
        .ok_or(de::Error::custom(
            "Could not access chars 5 and 6 of date (second). The string should be exactly 6 chars long.",
        ))?
        .parse::<u32>()
        .map_err(|e| de::Error::custom(format!("Error parsing day into integer: {e}")))?;

    Ok(
        NaiveTime::from_hms_opt(hour, minute, second).ok_or(de::Error::custom(
            format!("Could not convert hour ({hour}), minute ({minute}), and second ({second}), into NaiveTime"),
        ))?,
    )
}

pub fn ser_naivetime<S>(x: &NaiveTime, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let hour = x.hour();
    let minute = x.minute();
    let second = x.second();
    s.serialize_str(&format!("{:0>2}{:0>2}{:0>2}", hour, minute, second))
}

pub fn de_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let splits: Vec<&str> = s.split(':').collect();

    if splits.len() != 3 {
        return Err(de::Error::custom(format!(
            "Number of splits incorrect: {:?}",
            splits
        )));
    }

    let hours = splits[0].parse().map_err(|e| {
        de::Error::custom(format!("Cannot convert {} to integer: {e}", splits[0]))
    })?;
    let minutes = splits[1].parse().map_err(|e| {
        de::Error::custom(format!("Cannot convert {} to integer: {e}", splits[1]))
    })?;
    let seconds = splits[2].parse().map_err(|e| {
        de::Error::custom(format!("Cannot convert {} to integer: {e}", splits[2]))
    })?;

    if minutes > 60 || seconds > 60 {
        return Err(de::Error::custom(format!(
            "Minutes ({minutes}) or seconds ({seconds}) above 60"
        )));
    }

    Ok(Duration::hours(hours) + Duration::minutes(minutes) + Duration::seconds(seconds))
}

pub fn ser_duration<S>(x: &Duration, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&format!(
        "{:0>2}:{:0>2}:{:0>2}:",
        x.num_hours(),
        x.num_minutes() % 60,
        x.num_seconds() % 60
    ))
}
