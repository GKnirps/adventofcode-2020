use core::num::ParseIntError;
use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(&Path::new(&filename)).map_err(|e| e.to_string())?;
    let lines: Vec<&str> = content.split('\n').collect();
    let numbers: Vec<i32> = lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<i32>())
        .collect::<Result<Vec<i32>, ParseIntError>>()
        .map_err(|e| e.to_string())?;

    if let Some(result_1) = solve_part_1(&numbers) {
        println!("Answer to puzzle 1 is {}", result_1);
    } else {
        println!("There is no answer to puzzle 1");
    }

    Ok(())
}

fn solve_part_1(numbers: &[i32]) -> Option<i32> {
    for i in 0..(numbers.len() - 1) {
        for j in (i + 1)..numbers.len() {
            if numbers[i] + numbers[j] == 2020 {
                return Some(numbers[i] * numbers[j]);
            }
        }
    }
    None
}
