// بسم الله الرحمن الرحيم

use std::{collections::HashMap, env};
use {ureq, scraper};

// Some Constants
const BASE_URL: &str = "https://salah.com";
const CURRENT_VERSION: &str = "0.1.0";

fn fetch_source(url: String) -> String {
    let response = ureq::get(&url).call().unwrap();
    let _text = response.into_string().unwrap();

    return _text;
}

fn scrape_salah_times(text: String) -> HashMap<String, String> {
    let _document = scraper::Html::parse_document(&text);

    // Defining Selectors

    let fajr_time_selector = scraper::Selector::parse("dd#Fajr").unwrap();
    let dhuhr_time_selector = scraper::Selector::parse("dd#Dhuhr").unwrap();
    let asr_time_selector = scraper::Selector::parse("dd#Asr").unwrap();
    let maghrib_time_selector = scraper::Selector::parse("dd#Maghrib").unwrap();
    let isha_time_selector = scraper::Selector::parse("dd#Isha").unwrap();

    let location_selector = scraper::Selector::parse("h2>span").unwrap();

    let method_selector = scraper::Selector::parse("h4").unwrap();

    // Defining Elements

    let fajr_element = _document.select(&fajr_time_selector).next().unwrap();
    let dhuhr_element = _document.select(&dhuhr_time_selector).next().unwrap();
    let asr_element = _document.select(&asr_time_selector).next().unwrap();
    let maghrib_element = _document.select(&maghrib_time_selector).next().unwrap();
    let isha_element = _document.select(&isha_time_selector).next().unwrap();

    let location_element = _document.select(&location_selector).next().unwrap();

    let method_element = _document.select(&method_selector).next().unwrap();

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

    return _hashmap;
}

fn display_times(hashmap: HashMap<String, String>, hide: bool) {
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

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        if args[1] == "--help" {
            println!(
                r#"awqat - Simple CLI Application that fetches Salah times from salah.com

Usage: awqat [OPTIONS]

Options:
-V, --version       Print version info
-S, --show-location Run with showing the location
    --help          Show this help message"#
            );
        } else if args[1] == "-V" || args[1] == "--version" {
            println!("awqat {}\nWritten by Husayn al-Qurashi.", CURRENT_VERSION);
        } else if args[1] == "-S" || args[1] == "--show-location" {
            println!("Fetching Salah times...");
            let text = fetch_source(BASE_URL.to_string());
            let hashmap = scrape_salah_times(text);
            display_times(hashmap, false);
        }
    } else {
        println!("Fetching Salah times...");
        let text = fetch_source(BASE_URL.to_string());
        let hashmap = scrape_salah_times(text);
        display_times(hashmap, true);
    }
}

// تم بحمد الله
