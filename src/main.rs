// بسم الله الرحمن الرحيم

use std::{collections::HashMap, env};

// Some Constants
const BASE_URL: &str = "https://salah.com";
const CURRENT_VERSION: &str = "v0.1.3";

// Start defining Functions
fn fetch_source(url: &str) -> String {
    let response = ureq::get(url).call().expect("Failed to fetch URL");

    response.into_string().expect("Failed to get response text")
}

fn scrape_times(text: &str) -> HashMap<String, String> {
    let _document = scraper::Html::parse_document(text);

    // Defining Selectors

    let fajr_time_selector =
        scraper::Selector::parse("dd#Fajr").expect("Failed to find Fajr <dd> element");
    let dhuhr_time_selector =
        scraper::Selector::parse("dd#Dhuhr").expect("Failed to find Dhuhr <dd> element");
    let asr_time_selector =
        scraper::Selector::parse("dd#Asr").expect("Failed to find Asr <dd> element");
    let maghrib_time_selector =
        scraper::Selector::parse("dd#Maghrib").expect("Failed to find Maghrib <dd> element");
    let isha_time_selector =
        scraper::Selector::parse("dd#Isha").expect("Failed to find Isha <dd> element");

    let location_selector =
        scraper::Selector::parse("h2>span").expect("Failed to find Location <span> element");

    let method_selector =
        scraper::Selector::parse("h4").expect("Failed to find Method <h4> element");

    // Defining Elements

    let fajr_element = _document
        .select(&fajr_time_selector)
        .next()
        .expect("Failed to select Fajr <dd> element");
    let dhuhr_element = _document
        .select(&dhuhr_time_selector)
        .next()
        .expect("Failed to select Dhuhr <dd> element");
    let asr_element = _document
        .select(&asr_time_selector)
        .next()
        .expect("Failed to select Asr <dd> element");
    let maghrib_element = _document
        .select(&maghrib_time_selector)
        .next()
        .expect("Failed to select Maghrib <dd> element");
    let isha_element = _document
        .select(&isha_time_selector)
        .next()
        .expect("Failed to select Isha <dd> element");

    let location_element = _document
        .select(&location_selector)
        .next()
        .expect("Failed to select Location <span> element");

    let method_element = _document
        .select(&method_selector)
        .next()
        .expect("Failed to select Method <h4> element");

    // Now scrape the values
    // 1. Each Salah Time

    let times_elements_vector = vec![
        fajr_element,
        dhuhr_element,
        asr_element,
        maghrib_element,
        isha_element,
    ];

    let mut _hashmap: HashMap<String, String> = HashMap::new();

    let mut current_index = 0;

    for time_element in times_elements_vector {
        current_index += 1;

        let mut _time_name = "";

        match current_index {
            1 => _time_name = "fajr",
            2 => _time_name = "dhuhr",
            3 => _time_name = "asr",
            4 => _time_name = "maghrib",
            5 => _time_name = "isha",
            _ => _time_name = "invalid",
        }

        let time_value = time_element
            .inner_html()
            .replace("<span>", "")
            .replace("</span>", " ")
            .trim()
            .to_string();

        _hashmap.insert(_time_name.to_string(), time_value);
    }

    // 2. Location

    let location = location_element.inner_html().trim().to_string();

    _hashmap.insert("location".to_string(), location);

    // 3. Method

    let method = method_element
        .inner_html()
        .replace("<span>Method</span>", "")
        .trim()
        .to_string();

    _hashmap.insert("method".to_string(), method);

    // Return the HashMap

    _hashmap
}

fn format_times(hashmap: &HashMap<String, String>, hide: bool) {
    if hide {
        println!(
            "-------------------\nFajr     : {}\nDhuhr    : {}\n'Asr     : {}\nMaghrib  : {}\n'Isha    : {}\n-------------------\nMethod   : {}",
            hashmap["fajr"], hashmap["dhuhr"], hashmap["asr"], hashmap["maghrib"], hashmap["isha"], hashmap["method"]
        );
    } else {
        println!(
            "-------------------\nFajr     : {}\nDhuhr    : {}\n'Asr     : {}\nMaghrib  : {}\n'Isha    : {}\n-------------------\nLocation : {}\nMethod   : {}",
            hashmap["fajr"], hashmap["dhuhr"], hashmap["asr"], hashmap["maghrib"], hashmap["isha"], hashmap["location"], hashmap["method"]
        );
    }
}

fn display_times(hide: bool) {
    println!("Fetching Salah times...");
    let text = fetch_source(BASE_URL);
    let hashmap = scrape_times(&text);
    format_times(&hashmap, hide);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        match args.get(1).map(String::as_str) {
            Some("-H") | Some("--help") => {
                println!(
                    r#"awqat - Simple CLI Application that fetches Salah times from salah.com
    
Usage: awqat [OPTIONS]

Options:
    -V, --version       Print version info
    -S, --show-location Run with showing the location
    -H, --help          Show this help message

Amad Project: https://codeberg.org/amad"#
                );
            }
            Some("-V") | Some("--version") => {
                println!("awqat {}\nAmad Project: https://codeberg.org/amad", CURRENT_VERSION);
            }
            Some("-S") | Some("--show-location") => display_times(false),
            _ => display_times(true),
        }
    } else {
        display_times(true)
    }
}

// تم بحمد الله