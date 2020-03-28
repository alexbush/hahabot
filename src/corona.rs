use std::error::Error;
use serde::Deserialize;
use chrono::prelude::*;


#[derive(Debug, Deserialize)]
struct Latest {
    cases: u32,
    deaths: u32,
    recovered: u32,
    updated: i64,
    active: u32,
}

pub fn latest() -> Result<String, Box<dyn Error>> {
    let c: Latest =  
        reqwest::get("https://corona.lmao.ninja/all")?
        .json()?;

    let mut dt: DateTime<Utc> = Utc::now();
    if c.updated > 0 {
        dt = Utc.timestamp(c.updated / 1000, 0);
    }

    Ok(format!("cases: {}, deaths: {}, recovered: {}, active: {}\n --updated: {}", 
            c.cases, c.deaths, c.recovered, c.active, 
            dt.format("%Y-%m-%d %H:%M:%S").to_string()))
}

