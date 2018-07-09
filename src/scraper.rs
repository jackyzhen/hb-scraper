use reqwest;
use select::document::Document;
use select::predicate::{Class, Name};
use std::collections::HashMap;

const ALL_SHOWS: &str = "https://horriblesubs.info/shows";
const CURRENT_SEASON: &str = "https://horriblesubs.info/current-season";

pub struct Scraper {
    pub season_index: HashMap<String, String>,
    pub all_index: HashMap<String, String>,
}

impl Scraper {
    pub fn new() -> Scraper {
        Scraper {
            season_index: Scraper::index(CURRENT_SEASON),
            all_index: Scraper::index(ALL_SHOWS),
        }
    }

    fn index(url: &str) -> HashMap<String, String> {
        let mut map = HashMap::new();
        let resp = reqwest::get(url).unwrap();
        assert!(resp.status().is_success());
        let document = Document::from_read(resp).unwrap();

        for node in document.find(Class("ind-show")) {
            if let Some(a_ref) = node.find(Name("a")).next() {
                if let Some(url) = a_ref.attr("href") {
                    map.entry(a_ref.text()).or_insert(url.to_string());
                }
            }
        }
        println!("Loaded {} entries for {}", map.len(), url);
        map
    }

    pub fn search_all(&self, term: &str) -> Vec<(&str, &str)> {
        self.all_index
            .iter()
            .filter(|(k, _)| {
                k.to_lowercase()
                    .as_str()
                    .contains(term.to_lowercase().as_str())
            })
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect()
    }

    pub fn search_season(&self, term: &str) -> Vec<(&str, &str)> {
        self.season_index
            .iter()
            .filter(|(k, _)| {
                k.to_lowercase()
                    .as_str()
                    .contains(term.to_lowercase().as_str())
            })
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect()
    }
}
