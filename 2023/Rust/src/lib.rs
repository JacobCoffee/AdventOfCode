use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub mod days;
pub mod puzzle;

pub use puzzle::Puzzle;

/// Advent of Code 2023 CLI
#[derive(Parser)]
pub struct Cli {
    /// Day to run (1-25) (Optional, defaults to all available days)
    pub day: Option<usize>,
    /// Part of the day to run (Optional, defaults to both)
    #[clap(short = 'p', long)]
    pub part: Option<usize>,
}

pub fn run_day(day: usize, part: Option<usize>) -> Result<(), String> {
    let input = get_day_input(day).map_err(|e| e.to_string())?;

    let problem: Box<dyn Puzzle> = match day {
        1 => Box::new(days::day1::DayOne {}),
        2 => Box::new(days::day2::DayTwo {}),
        _ => return Err(format!("Day {} not implemented", day)),
    };

    match part {
        Some(1) => println!("{}", problem.part_one(&input)),
        Some(2) => println!("{}", problem.part_two(&input)),
        None => {
            println!("{}", problem.name());
            println!("{}", "-".repeat(problem.name().len()));
            println!("Part 1: {}", problem.part_one(&input));
            println!("Part 2: {}", problem.part_two(&input));
        }
        _ => return Err("Invalid part".to_string()),
    };

    Ok(())
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
