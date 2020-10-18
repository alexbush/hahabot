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
        Ok(reqwest::get(&format!("{}/v2/all?yesterday={}", &self.url, y))?.json()?)
    }

    fn countries(&self, sort_by: String, y: bool) -> Result<Vec<Country>, Box<dyn Error>> {
        Ok(reqwest::get(&format!("{}/v2/countries?sort={}&yesterday={}",
                    &self.url, sort_by, y))?.json()?)
    }

    fn country(&self, country: String) -> Result<Country, Box<dyn Error>> {
        let u = format!("{}/v2/countries/{}", &self.url, country);
        Ok(reqwest::get(&u)?.json()?)
    }
}

#[derive(Debug, Deserialize)]
struct Latest {
    updated: i64,
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
    cases_per_one_million: i64,
    #[serde(rename = "deathsPerOneMillion")]
    deaths_per_one_million: i64,
    tests: i64,
    #[serde(rename = "testsPerOneMillion")]
    tests_per_one_million: f64,
    #[serde(rename = "affectedCountries")]
    affected_countries: i64,
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
    let c = Api::new().all(false)?;
    let y = Api::new().all(true)?;

    let mut dt: DateTime<Utc> = Utc::now();
    if c.updated > 0 {
        dt = Utc.timestamp(c.updated / 1000, 0);
    }

    Ok(format!("```
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
Country:   {}
cases:     {} [+{}]
deaths:    {} [+{}]
recovered: {}
active:    {}
critical:  {}
--
{}```",
        c.country, c.cases, c.today_cases,
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
