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

    fn all(&self, y: bool) -> Result<Latest, Box<dyn Error>> {
//         Ok(reqwest::get(&format!("{}/v2/all?yesterday={}", &self.url, y))?.json()?)
        Ok(reqwest::get(&format!("{}/v3/covid-19/all?yesterday={}", &self.url, y))?.json()?)
    }

    fn countries(&self, sort_by: String, y: bool) -> Result<Vec<Country>, Box<dyn Error>> {
        Ok(reqwest::get(&format!("{}/v3/covid-19/countries?sort={}&yesterday={}", 
                    &self.url, sort_by, y))?.json()?)
    }

    fn country(&self, country: String) -> Result<Country, Box<dyn Error>> {
        let u = format!("{}/v3/covid-19/countries/{}?strict=true", &self.url, country);
        Ok(reqwest::get(&u)?.json()?)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Latest {
    population: i64,
    updated: i64,
    cases: i64,
    today_cases: i64,
    deaths: i64,
    today_deaths: i64,
    recovered: i64,
    active: i64,
    critical: i64,
    cases_per_one_million: Option<f64>,
    deaths_per_one_million: Option<f64>,
    tests: i64,
    tests_per_one_million: f64,
    affected_countries: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Country {
    pub updated: i64,
    pub country: String,
    pub country_info: CountryInfo,
    pub cases: i64,
    pub today_cases: i64,
    pub deaths: i64,
    pub today_deaths: i64,
    pub recovered: i64,
    pub today_recovered: i64,
    pub active: i64,
    pub critical: i64,
    pub cases_per_one_million: f64,
    pub deaths_per_one_million: f64,
    pub tests: i64,
    pub tests_per_one_million: f64,
    pub population: i64,
    pub continent: String,
    pub one_case_per_people: f64,
    pub one_death_per_people: f64,
    pub one_test_per_people: i64,
    pub active_per_one_million: f64,
    pub recovered_per_one_million: f64,
    pub critical_per_one_million: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CountryInfo {
    #[serde(rename = "_id")]
    pub id: Option<i64>,
    pub iso2: Option<String>,
    pub iso3: Option<String>,
    pub lat: f64,
    pub long: f64,
    pub flag: String,
}

pub fn latest() -> Result<String, Box<dyn Error>> {
    let c = Api::new().all(false)?;
    let y = Api::new().all(true)?;

    let mut dt: DateTime<Utc> = Utc::now();
    if c.updated > 0 {
        dt = Utc.timestamp(c.updated / 1000, 0);
    }

    Ok(format!("```
outbreak:  {:.5}%
population:{}

cases:     {} [+{}]
yesterday: {}

deaths:    {} [+{}]
yesterday: {}

recovered: {}
active:    {}
critical:  {}

affected countries: {}
--
{}```",
            c.active as f64 / c.population as f64 * 100.0,
            c.population,
            c.cases, 
            c.today_cases, 
            y.cases,
            c.deaths,
            c.today_deaths,
            y.deaths,
            c.recovered,
            c.active,
            c.critical,
            c.affected_countries,
            dt.format("%Y-%m-%d %H:%M:%S").to_string()))
}

pub fn latest_country(country: &String) -> Result<String, Box<dyn Error>> {
    let c = Api::new().country(country.to_string())?;

    let mut dt: DateTime<Utc> = Utc::now();
    if c.updated > 0 {
        dt = Utc.timestamp(c.updated / 1000, 0);
    }

    Ok(format!("```
outbreak:  {:.5}%
Country:   {}
population:{}
cases:     {} [+{}]
deaths:    {} [+{}]
recovered: {}
active:    {}
critical:  {}
--
{}```", 
        c.active as f64 / c.population as f64 * 100.0,
        c.country, c.population, c.cases, c.today_cases, 
        c.deaths, c.today_deaths, c.recovered,
        c.active, c.critical, dt.format("%Y-%m-%d %H:%M:%S").to_string()))
}


pub fn top(by: String) -> Result<String, Box<dyn Error>> {
    let mut result: String = "".to_string();

    if by == "help" {
        result = format!("Available sort values: cases, todayCases, deaths, todayDeaths");
    } else {
        let all = Api::new().countries(by, false)?;

        result.push_str("```\n");

        let top_5 = all[..5].to_vec();
        for a in top_5 {
            result.push_str(&format!("{}: c: {} [+{}], d: {} [+{}]\n",
                    a.country, a.cases, a.today_cases, a.deaths, a.today_deaths));
        }

        result.push_str("\n```");
    }

    Ok(result)
}
