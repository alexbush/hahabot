use std::error::Error;
use std::io::prelude::*;
use htmlescape::decode_html;
use select::predicate::{ Class, Name };
use select::document::Document;
use chrono::{ Utc, Datelike };
use reqwest::get;

// cache
struct Dtp {
    last_update: u32,
    header:      String,
    body:        String
}

static mut DTP_INFO: Dtp = Dtp {
    last_update: 0,
    header:      String::new(),
    body:        String::new()
};

pub fn ithappens() -> Result<String, Box<dyn Error>> {
    let mut res = get("https://ithappens.me/random")?;
    let mut buffer = String::new();
    res.read_to_string(&mut buffer)?;
    let html: &str = &buffer;
    let document = Document::from(html);
    let quote = document
        .find(Class("text"))
        .next()
        .unwrap();

    Ok(quote.text())
}


pub fn anekdot() -> Result<String, Box<dyn Error>> {
    let mut res = get("https://www.anekdot.ru/random/anekdot")?;
    let mut buffer = String::new();
    res.read_to_string(&mut buffer)?;
    let html: &str = &buffer;
    let document = Document::from(html);
    let mut quote = document
        .find(Class("text"))
        .next()
        .unwrap()
        .inner_html();

    quote = quote.replace("<br>", "\n");
    Ok(quote)
}


pub fn bash(id: u64) -> Result<String, Box<dyn Error>> {
    let mut res;
    if id != 0 {
        let url = format!("https://bash.im/quote/{}", id);
        res = get(&url)?;
    } else {
        res = get("https://bash.im/random")?;
    }
    let mut buffer = String::new();
    res.read_to_string(&mut buffer)?;
    let html: &str = &buffer;
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

pub fn dtp() -> Result<String, Box<dyn Error>> {
    let now = Utc::now();

    unsafe {
        if DTP_INFO.last_update != 0 &&
            (DTP_INFO.last_update == now.day() ||
            now.weekday().number_from_monday() > 5)
        {
            let mut result: String = DTP_INFO.header.clone();
            result.push_str(&format!("\n{}", DTP_INFO.body.as_str()));
            return Ok(result);
        }
    }

    let mut res = get("https://xn--90adear.xn--p1ai")?;
    let mut buffer = String::new();

    res.read_to_string(&mut buffer)?;

    let html: &str = &buffer;
    let document = Document::from(html);

    let quote = document
        .find(Class("b-crash-stat"))
        .next()
        .unwrap();

    let mut result = String::new();
    let mut header = String::new();
    let mut body = String::new();

    header.push_str(&format!("{}\n", quote.find(Name("th")).next().unwrap().text().trim()));

    unsafe {
        DTP_INFO.last_update = now.day();
        DTP_INFO.header = header.clone();
    }

    let mut i = 0;
    quote.find(Name("td")).for_each(|x| {
        if i & 1 == 0 {
            body.push_str(&format!("{}: ", x.text()));
        } else {
            body.push_str(&format!("{}\n", x.text()));
        }
        i = i + 1;
    });

    unsafe {
        DTP_INFO.body = body.clone();
    }

    result.push_str(&format!("{}\n", header.as_str()));
    result.push_str(&format!("{}", body.as_str()));

    Ok(result)
}
