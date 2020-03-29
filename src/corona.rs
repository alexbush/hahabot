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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Country {
    pub country: String,
    #[serde(rename = "countryInfo")]
    pub country_info: CountryInfo,
    pub cases: i64,
    #[serde(rename = "todayCases")]
    pub today_cases: i64,
    pub deaths: i64,
    #[serde(rename = "todayDeaths")]
    pub today_deaths: i64,
    pub recovered: i64,
    pub active: i64,
    pub critical: i64,
    #[serde(rename = "casesPerOneMillion")]
    pub cases_per_one_million: f64,
    #[serde(rename = "deathsPerOneMillion")]
    pub deaths_per_one_million: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CountryInfo {
    #[serde(rename = "_id")]
    pub id: i64,
    pub lat: f64,
    pub long: f64,
    pub flag: String,
    pub iso3: String,
    pub iso2: String,
}

pub fn latest_country(country: &String) -> Result<String, Box<dyn Error>> {
    let url = format!("https://corona.lmao.ninja/countries/{}", country);
    let c: Country = reqwest::get(&url)?.json()?;
    
    Ok(format!(r#"Country: {}
cases: {} [today: {}]
deaths: {} [today: {}]
recovered: {}
active: {}
critical: {}"#, 
        c.country, c.cases, c.today_cases, 
        c.deaths, c.today_deaths, c.recovered,
        c.active, c.critical))
}
