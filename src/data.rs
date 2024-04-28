use std::path::PathBuf;

use serde::de::Error;

use crate::{model::Score, Config};

pub fn parse_scores_dir() -> anyhow::Result<Vec<Score>> {
    let score_paths = read_scores_dir()?;
    let mut scores = score_paths.map(|p| -> anyhow::Result<Score> {
        let score: Score = serde_json::from_reader(
            std::fs::OpenOptions::new().read(true).open(&p)?,
        )
        .map_err(|e| serde_json::Error::custom(format!("{} in {}", e, p.to_str().unwrap())))?;
        Ok(score)
    });

    let ok_scores: anyhow::Result<Vec<Score>> = scores.try_fold(Vec::new(), |mut acc, x| {
        acc.push(x?);
        Ok(acc)
    });

    // let (error_scores, ok_scores): (Vec<_>, Vec<_>) = scores.partition(|s| s.is_err());

    // match error_scores.into_iter().next() {
    //     Some(s) => return Err(s.unwrap_err()),
    //     None => {}
    // }

    ok_scores

    // WHAT THE FUCK IS HAPPENING
}

pub fn read_scores_dir() -> anyhow::Result<impl Iterator<Item = PathBuf>> {
    let paths = std::fs::read_dir(read_config()?.scores_path)?
        .filter_map(|x| x.ok())
        .map(|x| x.path())
        .filter_map(|p| {
            if p.extension().map_or(false, |ext| ext == "json")
            /* Can't this be a map()? */
            {
                Some(p)
            } else {
                None
            }
        });

    Ok(paths)
}

pub fn read_config() -> anyhow::Result<Config> {
    let file = std::fs::OpenOptions::new().read(true).open("config.json")?;
    let config: Config = serde_json::from_reader(file)?;
    return Ok(config);
}
