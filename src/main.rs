use scraper::{Html, Selector};

fn main() {
    // Get HTML from URL
    let url = "https://en.wikipedia.org/wiki/2023_in_stand-up_comedy";
    let response = reqwest::blocking::get(url).expect("Could not load url.");
    let html = response.text().unwrap();

    // Get all <li>
    let document = Html::parse_document(&html);
    let selector = Selector::parse("li").unwrap();

    // Return lines beginning in a month
    for element in document.select(&selector) {
        let text = element.text().collect::<String>();
        if let Some(first_word) = text.split_whitespace().next() {
            if is_month(first_word) {
                let truncated_text = truncate_after_last_period(&text);
                let cleaned_text = truncated_text.replace('"', "");
                println!("{}", cleaned_text);
            }
        }
    }
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

fn truncate_after_last_period(text: &str) -> String {
    if let Some(index) = text.rfind('.') {
        text[..=index].to_string()
    } else {
        text.to_string()
    }
}
