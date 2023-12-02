use crate::puzzle::Puzzle;
use std::collections::HashMap;

pub struct DayOne;

use crate::number_word_map;

impl Puzzle for DayOne {
    fn name(&self) -> String {
        "Day 1: Trebuchet?!".to_string()
    }
    /// Prompt:
    /// As they're making the final adjustments, they discover that their calibration document
    /// (your puzzle input) has been amended by a very young Elf who was apparently just excited
    /// to show off her art skills. Consequently, the Elves are having trouble reading the
    /// values on the document.
    ///
    /// The newly-improved calibration document consists of lines of text;
    /// each line originally contained a specific calibration value that the Elves now
    /// need to recover. On each line, the calibration value can be found by combining
    /// the first digit and the last digit (in that order) to form a single two-digit number.
    ///
    /// In this example, the calibration values of these four lines are 12, 38, 15, and 77.
    /// Adding these together produces 142.
    ///
    /// Consider your entire calibration document.
    /// What is the sum of all of the calibration values?
    fn part_one(&self, input: &str) -> String {
        let input_first_last: Vec<(Option<char>, Option<char>)> = input
            .lines()
            .map(|line| {
                let numbers: Vec<char> = line.chars().filter(|c| c.is_ascii_digit()).collect();
                let first_number = numbers.first().cloned();
                let last_number = numbers.last().cloned();
                (first_number, last_number)
            })
            .collect();

        let mut sum = 0;
        for (first, last) in input_first_last {
            if let (Some(f), Some(l)) = (first, last) {
                let combined_str = format!("{}{}", f, l);

                if let Ok(num) = combined_str.parse::<u32>() {
                    sum += num;
                }
            }
        }
        sum.to_string()
    }

    /// Prompt:
    /// Your calculation isn't quite right. It looks like some of the digits are actually
    /// spelled out with letters: one, two, three, four, five, six, seven, eight, and
    /// nine also count as valid "digits".
    ///
    /// Equipped with this new information, you now need to find the real first and
    /// last digit on each line. For example:
    ///
    /// two1nine
    /// eightwothree
    /// abcone2threexyz
    /// xtwone3four
    /// 4nineeightseven2
    /// zoneight234
    /// 7pqrstsixteen
    ///
    /// In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76.
    /// Adding these together produces 281.
    ///
    /// What is the sum of all of the calibration values?
    fn part_two(&self, input: &str) -> String {
        let number_map = number_word_map();
        input
            .lines()
            .map(|line| numberize(line, &number_map))
            .sum::<usize>()
            .to_string()
    }

    fn input_path(&self) -> String {
        todo!()
    }
}

fn numberize(s: &str, number_map: &HashMap<String, u32>) -> usize {
    let digit_strings: Vec<String> = (1..=9).map(|i| i.to_string()).collect();
    let nums = number_map.iter().map(|(k, &v)| (k.as_str(), v)).chain(
        digit_strings
            .iter()
            .map(|s| (s.as_str(), s.parse::<u32>().unwrap())),
    );

    let (_, first) = nums
        .clone()
        .filter_map(|(n, i)| s.find(n).map(|loc| (loc, i)))
        .min_by_key(|&(loc, _)| loc)
        .unwrap();

    let (_, last) = nums
        .filter_map(|(n, i)| s.rfind(n).map(|loc| (loc, i)))
        .max_by_key(|&(loc, _)| loc)
        .unwrap();

    (first as usize) * 10 + (last as usize)
}

// Testing to see what each line looks like
// let as_strings: Vec<String> = input_first_last.iter()
//     .map(|(first, last)| {
//         format!(
//             "First: {}, Last: {}",
//             first.map_or("None".to_string(), |f| f.to_string()),
//             last.map_or("None".to_string(), |l| l.to_string())
//         )
//     })
//     .collect();

// Join the vector of strings into a single string separated by newlines
// as_strings.join("\n")
