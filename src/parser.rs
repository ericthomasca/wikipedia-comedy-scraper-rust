use chrono::NaiveDate;
use std::error::Error;
use crate::ComedySpecial;

pub fn parse_special(text: &str) -> Result<ComedySpecial, Box<dyn Error>> {
    let parts: Vec<&str> = text.split(':').collect();
    let date = parse_date(parts[0])?;
    let details = clean_text(parts[1])?;

    Ok(ComedySpecial { date, details })
}

fn clean_text(text: &str) -> Result<String, Box<dyn Error>> {
    let trimmed_text = if let Some(index) = text.rfind('.') {
        text[..=index].to_string()
    } else {
        text.to_string()
    };

    let cleaned_text = trimmed_text.replace('"', "").trim().to_string();

    Ok(cleaned_text)
}

pub fn parse_date(text: &str) -> Result<NaiveDate, Box<dyn Error>> {
    let parts: Vec<&str> = text.split(' ').collect();
    let month_str = parts[0].to_string();
    let month = parse_month(&month_str)?;
    let day = parts[1].parse::<u32>()?;

    NaiveDate::from_ymd_opt(2023, month, day).ok_or_else(|| "Unable to convert date".into())
}

pub fn parse_month(month_str: &str) -> Result<u32, Box<dyn Error>> {
    match month_str {
        "January" => Ok(1),
        "February" => Ok(2),
        "March" => Ok(3),
        "April" => Ok(4),
        "May" => Ok(5),
        "June" => Ok(6),
        "July" => Ok(7),
        "August" => Ok(8),
        "September" => Ok(9),
        "October" => Ok(10),
        "November" => Ok(11),
        "December" => Ok(12),
        _ => Err("Invalid month".into()),
    }
}

pub fn is_month(word: &str) -> bool {
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