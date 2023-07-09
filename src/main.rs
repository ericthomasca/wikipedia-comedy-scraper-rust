use chrono::NaiveDate;
use csv::Writer;
use scraper::{Html, Selector};
use std::error::Error;

struct ComedySpecial {
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
            if is_month(first_word) {
                let special = parse_special(&text);
                comedy_specials.push(special);
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

fn is_month(word: &str) -> bool {
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    months.iter().any(|&month| word.starts_with(month))
}

fn clean_text(text: &str) -> String {
    let trimmed_text = if let Some(index) = text.rfind('.') {
        text[..=index].to_string()
    } else {
        text.to_string()
    };

    trimmed_text.replace(['"', '“', '”'], "")
}

fn parse_special(text: &str) -> ComedySpecial {
    let parts: Vec<&str> = text.split(':').collect();
    let date = parse_date(parts[0]);
    let details = clean_text(parts[1]);

    ComedySpecial { date, details }
}

fn parse_date(text: &str) -> NaiveDate {
    let parts: Vec<&str> = text.split(' ').collect();
    let month_str = parts[0].to_string();
    let month = parse_month(&month_str);
    let day = parts[1].parse::<u32>().unwrap();

    NaiveDate::from_ymd_opt(2023, month, day).expect("Unable to convert date")
}

fn parse_month(month_str: &str) -> u32 {
    match month_str {
        "January" => 1,
        "February" => 2,
        "March" => 3,
        "April" => 4,
        "May" => 5,
        "June" => 6,
        "July" => 7,
        "August" => 8,
        "September" => 9,
        "October" => 10,
        "November" => 11,
        "December" => 12,
        _ => 0,
    }
}
