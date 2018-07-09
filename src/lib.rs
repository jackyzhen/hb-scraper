extern crate reqwest;
extern crate select;

pub mod fetcher;
pub mod scraper;

pub enum Quality {
    P480,
    P720,
    P1080,
}
