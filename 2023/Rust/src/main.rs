use advent_of_code_2023::days::day1::DayOne;
use advent_of_code_2023::get_day_input;
use advent_of_code_2023::{Cli, Puzzle};
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    let input = match get_day_input(cli.day) {
        Ok(data) => data,
        Err(err) => panic!("Error reading data for day {}: {}", cli.day, err),
    };

    let problem: Box<dyn Puzzle> = match cli.day {
        1 => Box::new(DayOne {}),
        _ => panic!("Day {} not implemented", cli.day),
    };

    match cli.part {
        Some(1) => println!("{}", problem.part_one(&input)),
        Some(2) => println!("{}", problem.part_two(&input)),
        None => {
            println!("{}", problem.name());
            println!("{}", "-".repeat(problem.name().len()));
            println!("Part 1: {}", problem.part_one(&input));
            println!("Part 2: {}", problem.part_two(&input));
        }
        _ => panic!("Invalid part"),
    };
}
