extern crate hs_scraper;

use hs_scraper as hs;
use std::io::{self, Write };
use std::fs::File;
use std::path::Path;

const HOST: &str = "https://horriblesubs.info";

fn main() {
    let (name, choice) = get_choice();
    let match_url = format!("{}{}", HOST, choice);
    let quality = get_quality();
    let fetcher = hs::fetcher::Fetcher::new(&match_url, &quality);
    let show_id = fetcher.get_show_id();
    let links = fetcher.get_mag_links(show_id);
    println!("{:#?}", links);
    write_links(&format!("{}-{}", name, &quality), links);
}

fn write_links(name: &str, links: Vec<String>) {
    let path = Path::new(name);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                          display,
                          why.to_string()),
        Ok(file) => file,
    };

    match file.write_all(links.join("\n").as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display,
                   why.to_string())
        },
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

fn get_quality() -> hs::Quality {
    let mut choice = String::new();
    println!("choose quality...");
    println!("1. 480p");
    println!("2. 720p (default)");
    println!("3. 1080p");
    print!("Choice:");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut choice)
        .expect("failed to read input");

    match choice.trim() {
        "1" | "1." | "480" | "480p" => hs::Quality::P480,
        "3" | "3." | "1080" | "1080o" => hs::Quality::P1080,
        _ => hs::Quality::P720,
    }
}

fn get_choice() -> (String, String) {
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
            let trimmed_choice = choice.trim();
            if scraper.all_index.contains_key(trimmed_choice) {
                return (trimmed_choice.to_string(),scraper.all_index.get(trimmed_choice).unwrap().to_string());
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
