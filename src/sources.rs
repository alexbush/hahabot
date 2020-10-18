use std::error::Error;
use std::io::prelude::*;
use htmlescape::decode_html;
use select::predicate::{ Class, Name };
use select::document::Document;

pub fn ithappens() -> Result<String, Box<dyn Error>> {
    let mut res = reqwest::get("https://ithappens.me/random")?;
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
    let mut res = reqwest::get("https://www.anekdot.ru/random/anekdot")?;
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
        res = reqwest::get(&url)?;
    } else {
        res = reqwest::get("https://bash.im/random")?;
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
    let mut res = reqwest::get("https://xn--90adear.xn--p1ai")?;
    let mut buffer = String::new();

    res.read_to_string(&mut buffer)?;

    let html: &str = &buffer;
    let document = Document::from(html);

    let quote = document
        .find(Class("b-crash-stat"))
        .next()
        .unwrap();

    let mut result = String::new();
    result.push_str(&format!("{}\n", quote.find(Name("th")).next().unwrap().text().trim()));

    let mut i = 0;
    quote.find(Name("td")).for_each(|x| {
        if i & 1 == 0 {
            result.push_str(&format!("{}: ", x.text()));
        } else {
            result.push_str(&format!("{}\n", x.text()));
        }
        i = i + 1;
    });

    Ok(result)
}
