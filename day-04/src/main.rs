use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(&Path::new(&filename)).map_err(|e| e.to_string())?;

    let pass_maps = parse_pass_maps(&content);

    let passes_with_all_fields = pass_maps
        .iter()
        .filter(|pass| is_pass_map_valid(pass))
        .count();

    println!(
        "There are {} passes that contain all fields (ignoring cid)",
        passes_with_all_fields
    );

    let valid_passes = pass_maps
        .iter()
        .filter(|pass| is_pass_map_data_valid(pass))
        .count();

    println!(
        "There are {} passes where all required fields are valid",
        valid_passes
    );

    Ok(())
}

fn is_pass_map_valid(pass: &HashMap<&str, &str>) -> bool {
    ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .all(|field| pass.contains_key(field))
}

fn is_pass_map_data_valid(pass: &HashMap<&str, &str>) -> bool {
    birth_year_valid(pass)
        && issue_year_valid(pass)
        && expiration_year_valid(pass)
        && height_valid(pass)
        && check_hair_color(pass)
        && check_eye_color(pass)
        && check_passport_id(pass)
}

fn birth_year_valid(pass: &HashMap<&str, &str>) -> bool {
    pass.get("byr")
        .map(|y| check_range(y, 1920, 2002))
        .unwrap_or(false)
}

fn issue_year_valid(pass: &HashMap<&str, &str>) -> bool {
    pass.get("iyr")
        .map(|y| check_range(y, 2010, 2020))
        .unwrap_or(false)
}

fn expiration_year_valid(pass: &HashMap<&str, &str>) -> bool {
    pass.get("eyr")
        .map(|y| check_range(y, 2020, 2030))
        .unwrap_or(false)
}

fn height_valid(pass: &HashMap<&str, &str>) -> bool {
    pass.get("hgt")
        .map(|s| {
            if s.ends_with("cm") {
                check_range(s.split_at(s.len() - 2).0, 150, 193)
            } else if s.ends_with("in") {
                check_range(s.split_at(s.len() - 2).0, 59, 76)
            } else {
                false
            }
        })
        .unwrap_or(false)
}

fn check_hair_color(pass: &HashMap<&str, &str>) -> bool {
    pass.get("hcl")
        .map(|s| s.starts_with('#') && s.split_at(1).1.chars().all(|c| c.is_ascii_hexdigit()))
        .unwrap_or(false)
}

fn check_eye_color(pass: &HashMap<&str, &str>) -> bool {
    pass.get("ecl")
        .map(|s| ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(s))
        .unwrap_or(false)
}

fn check_passport_id(pass: &HashMap<&str, &str>) -> bool {
    pass.get("pid")
        .map(|s| s.len() == 9 && s.chars().all(|c| c.is_ascii_digit()))
        .unwrap_or(false)
}

fn check_range(number: &str, min: i32, max: i32) -> bool {
    number
        .parse::<i32>()
        .ok()
        .map(|n| n >= min && n <= max)
        .unwrap_or(false)
}

fn parse_name_value_pair(input: &str) -> Option<(&str, &str)> {
    let mut parts = input.splitn(2, ':');
    let name = parts.next()?;
    let value = parts.next()?;
    Some((name, value))
}

fn parse_pass_map(input: &str) -> HashMap<&str, &str> {
    input
        .split_whitespace()
        .filter_map(parse_name_value_pair)
        .collect()
}

fn parse_pass_maps(input: &str) -> Vec<HashMap<&str, &str>> {
    input
        .split("\n\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(parse_pass_map)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
}
