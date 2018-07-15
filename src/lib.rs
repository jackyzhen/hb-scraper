extern crate reqwest;
extern crate select;

use std::fmt::{ Result, Formatter, Display};

pub mod fetcher;
pub mod scraper;

#[derive(Debug)]
pub enum Quality {
    P480,
    P720,
    P1080,
}

impl Display for Quality {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}
