use std::error::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Corona {
    latest: Latest,
}

#[derive(Debug, Deserialize)]
struct Latest {
    confirmed: u32,
    deaths: u32,
    recovered: u32,
}

pub fn corona() -> Result<String, Box<dyn Error>> {
    let c: Corona = 
        reqwest::get("https://coronavirus-tracker-api.herokuapp.com/v2/latest")?
        .json()?;

    Ok(format!("Confirmed: {} Deaths: {}", c.latest.confirmed, c.latest.deaths))
}
