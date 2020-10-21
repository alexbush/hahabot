use std::error::Error;
use serde::Deserialize;
use chrono::prelude::*;
use std::fmt;

struct Api { url: String }

impl Api {
    fn new() -> Self { Self { url: "https://disease.sh".to_string() } }

    fn all(&self, y: bool) -> Result<Covid, Box<dyn Error>> {
        Ok(reqwest::get(&format!("{}/v3/covid-19/all?yesterday={}", &self.url, y))?.json()?)
    }

    fn countries(&self, sort_by: String, y: bool) -> Result<Vec<Covid>, Box<dyn Error>> {
        Ok(reqwest::get(&format!("{}/v3/covid-19/countries?sort={}&yesterday={}", 
                    &self.url, sort_by, y))?.json()?)
    }
    
    fn country(&self, country: String) -> Result<Covid, Box<dyn Error>> {
        let u = format!("{}/v3/covid-19/countries/{}?strict=true", &self.url, country);
        Ok(reqwest::get(&u)?.json()?)
    }
    fn vaccine(&self) -> Result<Vaccine, Box<dyn Error>> {
        Ok( reqwest::get(&format!("{}/v3/covid-19/vaccine", &self.url))?.json()?)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Covid {
    active:                     i64,
    active_per_one_million:     f64,
    affected_countries:         Option<i64>,
    cases:                      i64,
    cases_per_one_million:      f64,
    continent:                  Option<String>,
    country:                    Option<String>,
    country_info:               Option<CountryInfo>,
    critical:                   i64,
    critical_per_one_million:   f64,
    deaths:                     i64,
    deaths_per_one_million:     Option<f64>,
    one_case_per_people:        f64,
    one_death_per_people:       f64,
    one_test_per_people:        i64,
    population:                 i64,
    recovered:                  i64,
    recovered_per_one_million:  f64,
    tests:                      i64,
    tests_per_one_million:      f64,
    today_cases:                i64,
    today_deaths:               i64,
    today_recovered:            i64,
    updated:                    i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CountryInfo {
    #[serde(rename = "_id")]
    id:    Option<i64>,
    iso2:  Option<String>,
    iso3:  Option<String>,
    lat:   f64,
    long:  f64,
    flag:  String,
}

impl Covid {
    fn outbreak(&self) -> f64 {
        if self.population > 0 {
            self.active as f64 / self.population as f64 * 100.0
        } else {
            0.0
        }
    }
    fn dt(&self) -> DateTime<Utc> {
        let mut dt: DateTime<Utc> = Utc::now();
        if self.updated > 0 {
            dt = Utc.timestamp(self.updated / 1000, 0);
        }
        dt
    }
}

impl fmt::Display for Covid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let country = match &self.country {
            Some(c) => c.clone(),
            None    => "World".to_string()
        };
        write!(f, "```
Country:   {}
population:{}
outbreak:  {:.5}%

cases:     {} [+{}]
deaths:    {} [+{}]

active:    {}
recovered: {}
critical:  {}
--
{}```", 
        country, 
        self.population, 
        self.outbreak(),
        self.cases, 
        self.today_cases, 
        self.deaths, 
        self.today_deaths, 
        self.active, 
        self.recovered,
        self.critical, 
        self.dt().format("%Y-%m-%d %H:%M:%S").to_string())
    }
}

pub fn latest(country: Option<String>) -> Result<String, Box<dyn Error>> {
    let covid = match country {
        Some(c) => Api::new().country(c)?,
        None    => Api::new().all(false)?,
    };
    Ok(format!("{}", covid))
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
            let country = match &a.country {
                Some(c) => c.clone(),
                None    => "".to_string(),
            };
            result.push_str(&format!("{}: c: {} [+{}], d: {} [+{}]\n", 
                    country, a.cases, a.today_cases, a.deaths, a.today_deaths));
        }

        result.push_str("\n```");
    }

    Ok(result)
}


#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Vaccine {
    source:            String,
    total_candidates:  String,
    phases:            Vec<Phase>,
    data:              Vec<VaccineData>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Phase {
    phase:       String,
    candidates:  String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct VaccineData {
    candidate:     String,
    mechanism:     String,
    sponsors:      Vec<String>,
    details:       String,
    trial_phase:   String,
    institutions:  Vec<String>,
}

impl fmt::Display for Vaccine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut phase: String = "".to_string();
        for p in &self.phases {
            phase.push_str(&format!("{:15} : {}\n", p.phase, p.candidates));
        }

        write!(f, "{}
```
total:   {}

phases:  
{}
```", 
        self.source
            .replace("-", "\\-")
            .replace(".", "\\."),
        self.total_candidates,
        phase.replace("-", "\\-"))
    }
}

pub fn vaccine() -> Result<String, Box<dyn Error>> {
    let v = Api::new().vaccine()?;
    Ok(format!("{}", v))
}
