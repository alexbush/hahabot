use std::error::Error;
use tokio::sync::Mutex;
use htmlescape::decode_html;
use select::predicate::{ Class, Name };
use select::document::Document;
use chrono::{ Utc, Datelike };
use reqwest::get;
use std::sync::Arc;

use super::DtpCache;

pub async fn ithappens() -> Result<String, Box<dyn Error>> {
    let content = get("https://ithappens.me/random")
        .await?
        .text()
        .await?;
    let html: &str = &content;
    let document = Document::from(html);
    let quote = document
        .find(Class("text"))
        .next()
        .unwrap();

    Ok(quote.text())
}


pub async fn anekdot() -> Result<String, Box<dyn Error>> {
    let content = get("https://www.anekdot.ru/random/anekdot").await?.text().await?;
    let html: &str = &content;
    let document = Document::from(html);
    let mut quote = document
        .find(Class("text"))
        .next()
        .unwrap()
        .inner_html();

    quote = quote.replace("<br>", "\n");
    Ok(quote)
}


pub async fn bash(id: u64) -> Result<String, Box<dyn Error>> {
    let url;
    if id != 0 {
        url = format!("https://bash.im/quote/{}", id)
    } else {
        url = String::from("https://bash.im/random")
    };
    let content = get(&url).await?.text().await?;
    let html: &str = &content;
    let document = Document::from(html);

    let quote_id = document
        .find(Class("quote__header_permalink"))
        .next()
        .unwrap()
        .inner_html();

    let mut quote = document
        .find(Class("quote__body"))
        .next()
        .unwrap()
        .inner_html();

    quote = quote.replace("<br>", "\n");
    let quote = match decode_html(quote.as_str()) {
        Err(reason) => panic!("Error {:?} at character {}", reason.kind, reason.position),
        Ok(s) => s
    };
    Ok(format!("{}\n{}", quote_id, quote))
}

pub async fn dtp(cache: &Arc<Mutex<DtpCache>>) -> Result<String, Box<dyn Error>> {
    let now = Utc::now();

    let mut dtp_cache = cache.lock().await;

    if dtp_cache.last_update != 0 &&
        (dtp_cache.last_update == now.day() ||
        (now.weekday().number_from_monday() > 5 &&
         dtp_cache.last_update >= now.day() - (now.weekday().number_from_monday() - 5)))
    {
        let mut result: String = dtp_cache.header.clone();
        result.push_str(&format!("\n{}", dtp_cache.body.as_str()));
        return Ok(result);
    }

    let content = get("https://xn--90adear.xn--p1ai").await?.text().await?;

    let html: &str = &content;
    let document = Document::from(html);

    let quote = document
        .find(Class("b-crash-stat"))
        .next()
        .unwrap();

    let mut result = String::new();
    let mut header = String::new();
    let mut body = String::new();

    header.push_str(&format!("{}\n", quote.find(Name("th")).next().unwrap().text().trim()));

    dtp_cache.last_update = now.day();
    dtp_cache.header = header.clone();

    let mut i = 0;
    quote.find(Name("td")).for_each(|x| {
        if i & 1 == 0 {
            body.push_str(&format!("{}: ", x.text()));
        } else {
            body.push_str(&format!("{}\n", x.text()));
        }
        i += 1;
    });

    dtp_cache.body = body.clone();

    result.push_str(&format!("{}\n", header.as_str()));
    result.push_str(&format!("{}", body.as_str())); // Can be rewritten with push_str(&body);

    Ok(result)
}
