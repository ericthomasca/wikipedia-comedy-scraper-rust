use chrono::NaiveDate;
use csv::Writer;
use scraper::{Html, Selector};
use std::error::Error;
use log::error;

mod parser;

pub struct ComedySpecial {
    date: NaiveDate,
    details: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Get HTML from URL
    let url = "https://en.wikipedia.org/wiki/2023_in_stand-up_comedy";
    let response = reqwest::blocking::get(url)?;
    let html = response.text()?;

    // Get all <li>
    let document = Html::parse_document(&html);
    let selector = Selector::parse("li")?;

    let mut comedy_specials: Vec<ComedySpecial> = Vec::new();

    // Return lines beginning with a month
    for element in document.select(&selector) {
        let text = element.text().collect::<String>();
        if let Some(first_word) = text.split_whitespace().next() {
            if parser::is_month(first_word) {
                if let Ok(special) = parser::parse_special(&text) {
                    comedy_specials.push(special);
                } else {
                    error!("Failed to parse special: {}", text);
                    continue;
                }
            }
        }
    }

    // Save to csv
    let mut writer = Writer::from_path("specials.csv")?;

    writer.write_record(["Date", "Details"])?;

    for special in comedy_specials {
        writer.write_record(&[special.date.to_string(), special.details])?;
    }

    println!("Data written to \"specials.csv\"");

    Ok(())
}






