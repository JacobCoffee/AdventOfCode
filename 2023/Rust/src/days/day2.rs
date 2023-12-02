use crate::Puzzle;
use std::collections::HashMap;
pub struct DayTwo;

impl Puzzle for DayTwo {
    fn name(&self) -> String {
        "Day 2: Cube Conundrum".to_string()
    }

    fn part_one(&self, input: &str) -> String {
        sum_of_possible_games(input).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        sum_of_powers(input).to_string()
    }

    fn input_path(&self) -> String {
        todo!()
    }
}

/// Day 2s is in the format of "Game $id: $set1; $set2; $set3'
fn parse_game_line(line: &str) -> (i32, Vec<HashMap<String, i32>>) {
    let parts: Vec<&str> = line.split(": ").collect();
    let id: i32 = parts[0].split_whitespace().nth(1).unwrap().parse().unwrap();
    let sets = parts[1]
        .split("; ")
        .map(|set| {
            set.split(", ")
                .map(|cube| {
                    let parts: Vec<&str> = cube.split_whitespace().collect();
                    let count: i32 = parts[0].parse().unwrap();
                    let color = parts[1].to_string();
                    (color, count)
                })
                .collect::<HashMap<_, _>>()
        })
        .collect();
    (id, sets)
}

fn is_game_possible(sets: &[HashMap<String, i32>]) -> bool {
    let max_cubes = HashMap::from([
        ("red".to_string(), 12),
        ("green".to_string(), 13),
        ("blue".to_string(), 14),
    ]);

    for set in sets {
        for (color, &count) in set {
            if count > *max_cubes.get(color).unwrap_or(&0) {
                return false;
            }
        }
    }
    true
}

fn sum_of_possible_games(input: &str) -> i32 {
    let mut sum = 0;
    for line in input.lines() {
        let (id, game_data) = parse_game_line(line);
        if is_game_possible(&game_data) {
            sum += id;
        }
    }
    sum
}

fn sum_of_powers(input: &str) -> i64 {
    input.lines().map(calculate_power).sum()
}

fn calculate_power(line: &str) -> i64 {
    let (_, game_data) = parse_game_line(line);
    let mut min_cubes = HashMap::new();

    for set in game_data {
        for (color, &count) in set.iter() {
            let entry = min_cubes.entry(color.clone()).or_insert(0);
            *entry = (*entry).max(count);
        }
    }

    min_cubes.values().map(|&val| val as i64).product()
}
