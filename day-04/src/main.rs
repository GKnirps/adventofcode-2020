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

    let valid_passes = pass_maps
        .iter()
        .filter(|pass| is_pass_map_valid(pass))
        .count();

    println!("There are {} valid passes (ignoring cid)", valid_passes);

    Ok(())
}

fn is_pass_map_valid(pass: &HashMap<&str, &str>) -> bool {
    ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .all(|field| pass.contains_key(field))
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
