use advent_of_code_2023::{run_day};
use advent_of_code_2023::{Cli};
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    match cli.day {
        Some(day) => {
            if let Err(e) = run_day(day, cli.part) {
                eprintln!("Error: {}", e);
            }
        }
        None => {
            for day in 1..=25 {
                // TODO: doesnt really work for nonexistent days :(
                if let Err(e) = run_day(day, cli.part) {
                    eprintln!("Error on day {}: {}", day, e);
                    break;
                }
            }
        }
    };
}
