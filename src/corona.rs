use std::error::Error;
use serde::Deserialize;
use chrono::prelude::*;

struct Api { url: String }

impl Api {
    fn new() -> Self {
        Self {
            url: "https://corona.lmao.ninja".to_string()
        }
    }
    
    fn by_country(&self, country: String) -> Result<Country, Box<dyn Error>> {
        let u = format!("{}/countries/{}", &self.url, country);
        Ok(reqwest::get(&u)?.json()?)
    }

    fn all(&self, by: String) -> Result<Vec<Country>, Box<dyn Error>> {
        let u = match by.is_empty() {
            true => format!("{}/countries?sort={}", &self.url, by),
            false => format!("{}/countries", &self.url),
        };

        Ok(reqwest::get(&u)?.json()?)
    }
}

#[derive(Debug, Deserialize)]
struct Latest {
    cases: u32,
    deaths: u32,
    recovered: u32,
    updated: i64,
    active: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
struct Country {
    country: String,
    #[serde(rename = "countryInfo")]
    country_info: CountryInfo,
    cases: i64,
    #[serde(rename = "todayCases")]
    today_cases: i64,
    deaths: i64,
    #[serde(rename = "todayDeaths")]
    today_deaths: i64,
    recovered: i64,
    active: i64,
    critical: i64,
    #[serde(rename = "casesPerOneMillion")]
    cases_per_one_million: Option<f64>,
    #[serde(rename = "deathsPerOneMillion")]
    deaths_per_one_million: Option<f64>,
    updated: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
struct CountryInfo {
    #[serde(rename = "_id")]
    id: Option<i64>,
    lat: Option<f64>,
    long: Option<f64>,
    flag: Option<String>,
    iso3: Option<String>,
    iso2: Option<String>,
}

pub fn latest() -> Result<String, Box<dyn Error>> {
    let c = Api::new().by_country("world".to_string())?;

    let mut dt: DateTime<Utc> = Utc::now();
    if c.updated > 0 {
        dt = Utc.timestamp(c.updated / 1000, 0);
    }

    Ok(format!("cases: {} [+{}], deaths: {} [+{}], recovered: {}, active: {}\n --updated: {}", 
            c.cases, c.today_cases, c.deaths, c.today_deaths, c.recovered, c.active, 
            dt.format("%Y-%m-%d %H:%M:%S").to_string()))
}

pub fn latest_country(country: &String) -> Result<String, Box<dyn Error>> {
    let c = Api::new().by_country(country.to_string())?;

    let mut dt: DateTime<Utc> = Utc::now();
    if c.updated > 0 {
        dt = Utc.timestamp(c.updated / 1000, 0);
    }
    
    Ok(format!(r#"Country: {}
cases: {} [today: {}]
deaths: {} [today: {}]
recovered: {}
active: {}
critical: {}
{}"#, 
        c.country, c.cases, c.today_cases, 
        c.deaths, c.today_deaths, c.recovered,
        c.active, c.critical, dt.format("%Y-%m-%d %H:%M:%S").to_string()))
}


pub fn top(by: String) -> Result<String, Box<dyn Error>> {
    let all = Api::new().all(by)?;

    let top_5 = all[..6].to_vec();
    let mut result: String = "".to_string();
    for a in top_5 {
        result.push_str(&format!("{}: c: {} [+{}], d: {} [+{}]\n", 
                a.country, a.cases, a.today_cases, a.deaths, a.today_deaths));
    }

    Ok(result)
}
