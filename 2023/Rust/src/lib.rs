use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub mod days;
pub mod problem;

pub use problem::Puzzle;

/// Advent of Code 2023 CLI
#[derive(Parser)]
pub struct Cli {
    /// Day to run (1-25)
    pub day: usize,
    /// Part of the day to run (Optional, defaults to both)
    #[clap(short = 'p', long)]
    pub part: Option<usize>,
}

/// Get the input from Data/day{day}.input for the given day
pub fn get_day_input(day: usize) -> Result<String, std::io::Error> {
    let data_file_path = Path::new("../Data").join(format!("day{}.input", day));
    fs::read_to_string(data_file_path)
}

pub fn output_day_input(day: usize) -> Result<(), String> {
    match get_day_input(day) {
        Ok(input) => {
            println!("{}", input);
            Ok(())
        }
        Err(err) => {
            println!("Error: {}", err);
            Err(err.to_string())
        }
    }
}

/// A representation of a number in word for for Day 1 Part 2
pub fn number_word_map() -> HashMap<String, u32> {
    let mut map = HashMap::new();
    map.insert("one".to_string(), 1);
    map.insert("two".to_string(), 2);
    map.insert("three".to_string(), 3);
    map.insert("four".to_string(), 4);
    map.insert("five".to_string(), 5);
    map.insert("six".to_string(), 6);
    map.insert("seven".to_string(), 7);
    map.insert("eight".to_string(), 8);
    map.insert("nine".to_string(), 9);
    map
}
