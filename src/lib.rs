use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn input(name: &str) -> io::Result<Vec<String>> {
    BufReader::new(File::open(Path::new("input").join(name))?)
        .lines()
        .collect()
}
