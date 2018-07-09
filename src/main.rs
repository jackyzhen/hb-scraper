extern crate hs_scraper;

use hs_scraper as hs;
use std::io::{self, Write};

const HOST: &str = "https://horriblesubs.info";

fn main() {
    let choice = get_choice();
    let match_url = format!("{}{}", HOST, choice);
    let fetcher = hs::fetcher::Fetcher::new(&match_url, hs::Quality::P1080);
    let show_id = fetcher.get_show_id();
    let links = fetcher.get_mag_links(show_id);
    println!("{:#?}", links);
}

fn get_choice() -> String {
    println!("Loading indexes...");
    let scraper = hs::scraper::Scraper::new();
    let mut choice = String::new();
    loop {
        let input = get_search();
        let matches = scraper.search_all(input.trim());
        println!("{:#?}", matches);
        if !continue_search() {
            print!("Choice: ");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut choice)
                .expect("failed to read input");
            if scraper.all_index.contains_key(choice.trim()) {
                return scraper.all_index.get(choice.trim()).unwrap().to_string();
            } else {
                println!("Choice does not match any index (use name of anime).")
            }
        }
    }
}

fn get_search() -> String {
    let mut input = String::new();
    print!("Search: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    input.trim().to_string()
}

fn continue_search() -> bool {
    let mut input = String::new();
    print!("Search again? <y,n>: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    match input.trim() {
        "y" => true,
        _ => false,
    }
}
