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
    pub cases_per_one_million: Option<f64>,
    #[serde(rename = "deathsPerOneMillion")]
    pub deaths_per_one_million: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CountryInfo {
    #[serde(rename = "_id")]
    pub id: Option<i64>,
    pub lat: Option<f64>,
    pub long: Option<f64>,
    pub flag: Option<String>,
    pub iso3: Option<String>,
    pub iso2: Option<String>,
}

pub fn latest() -> Result<String, Box<dyn Error>> {
    let c: Latest =  
        reqwest::get("https://corona.lmao.ninja/all")?
        .json()?;
    
    let all: Vec<Country> = reqwest::get("https://corona.lmao.ninja/countries")?.json()?;

    let mut today_deaths: i64 = 0;
    let mut today_cases: i64 = 0;
    for a in all {
        today_deaths = today_deaths + a.today_deaths;
        today_cases = today_cases + a.today_cases;
    }

    let mut dt: DateTime<Utc> = Utc::now();
    if c.updated > 0 {
        dt = Utc.timestamp(c.updated / 1000, 0);
    }

    Ok(format!("cases: {} [+{}], deaths: {} [+{}], recovered: {}, active: {}\n --updated: {}", 
            c.cases, today_cases, c.deaths, today_deaths, c.recovered, c.active, 
            dt.format("%Y-%m-%d %H:%M:%S").to_string()))
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


pub fn top(by: String) -> Result<String, Box<dyn Error>> {
    let url = format!("https://corona.lmao.ninja/countries?sort={}", by);
    let all: Vec<Country> = 
        reqwest::get(&url)?
        .json()?;

    let top_5 = all[..5].to_vec();
    let mut result: String = "".to_string();
    for a in top_5 {
        result.push_str(&format!("{}: c: {} [+{}], d: {} [+{}]\n", 
                a.country, a.cases, a.today_cases, a.deaths, a.today_deaths));
    }

    Ok(result)
}
