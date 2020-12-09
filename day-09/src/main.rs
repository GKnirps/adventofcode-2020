use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(&Path::new(&filename)).map_err(|e| e.to_string())?;
    let input: Vec<u64> = parse_input(&content)?;

    if let Some(invalid_number) = find_first_invalid_number(&input, 25) {
        println!("First invalid number is {}", invalid_number);
    } else {
        println!("There is no invalid number.");
    }

    Ok(())
}

fn find_first_invalid_number(data: &[u64], preamble_length: usize) -> Option<u64> {
    for window in data.windows(preamble_length + 1) {
        if !number_valid(&window[0..preamble_length], window[preamble_length]) {
            return Some(window[preamble_length]);
        }
    }
    None
}

// for this, brute forcing should be a viable option
fn number_valid(preamble: &[u64], number: u64) -> bool {
    for a in 0..(preamble.len() - 1) {
        for b in (a + 1)..preamble.len() {
            if preamble[a] + preamble[b] == number {
                return true;
            }
        }
    }
    false
}

fn parse_input(input: &str) -> Result<Vec<u64>, String> {
    input
        .split_terminator('\n')
        .map(|l| {
            l.parse::<u64>()
                .map_err(|e| format!("Unable to parse line '{}': {}", l, e))
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn find_first_invalid_number_works_for_example() {
        // given
        let data = [
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        let preamble_length = 5;

        // when
        let result = find_first_invalid_number(&data, preamble_length);

        // then
        assert_eq!(result, Some(127));
    }
}
