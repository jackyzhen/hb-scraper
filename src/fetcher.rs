use super::Quality;
use reqwest;
use select::document::Document;
use select::predicate::{Class, Name};

const API_TEMPLATE: &'static str =
    "https://horriblesubs.info/api.php?method=getshows&type=show&showid=";

pub struct Fetcher<'a> {
    url: &'a str,
    quality: Quality,
}

impl<'a> Fetcher<'a> {
    pub fn new(url: &'a str, quality: Quality) -> Fetcher<'a> {
        Fetcher { url, quality }
    }

    pub fn get_show_id(&self) -> usize {
        println!("getting show id from {}", self.url);
        let resp = reqwest::get(self.url).unwrap();
        assert!(resp.status().is_success());
        let doc = Document::from_read(resp).unwrap();
        let scripts = doc.find(Name("script"));
        let id_code = scripts
            .filter(|n| n.attr("type").is_some())
            .filter(|n| n.attr("type").unwrap() == "text/javascript")
            .filter_map(|n| {
                if n.text().contains("var hs_showid =") {
                    Some(n.text())
                } else {
                    None
                }
            })
            .next()
            .unwrap();

        let id = id_code
            .trim_left_matches("var hs_showid =")
            .trim_right_matches(";")
            .trim();
        println!("got hs id: {}", id);

        id.parse::<usize>().unwrap()
    }

    pub fn get_mag_links(&self, id: usize) -> Vec<String> {
        let url = format!("{}{}&nextid=", API_TEMPLATE, id);
        let mut links = Vec::new();
        let mut next_id = 0;
        loop {
            let next_url = format!("{}{}", url, next_id);
            println!("fetching mag links from {}", next_url);

            let resp = reqwest::get(&next_url).unwrap();
            assert!(resp.status().is_success());

            let document = Document::from_read(resp).unwrap();

            let quality_selector = match self.quality {
                Quality::P480 => "link-480",
                Quality::P720 => "link-720p",
                Quality::P1080 => "link-1080p",
            };

            if let Some(body) = document.find(Name("body")).next() {
                if body.text() == "DONE" {
                    println!("DONE");
                    break;
                }
            }

            for node in document.find(Class("rls-links-container")) {
                if let Some(link_container) = node.find(Class(quality_selector)).next() {
                    if let Some(link_span) = link_container.find(Class("hs-magnet-link")).next() {
                        if let Some(link) = link_span.find(Name("a")).next() {
                            links.push(link.attr("href").unwrap().to_string());
                        }
                    }
                }
            }

            next_id += 1;
        }
        links
    }
}
