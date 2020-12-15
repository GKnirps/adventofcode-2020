use std::collections::HashMap;

fn main() -> Result<(), String> {
    let input: &[usize] = &[14, 3, 1, 0, 9, 5];

    let number_2020 = get_number_2020(input)?;
    println!("Number at index #2020 is: {}", number_2020);

    Ok(())
}

fn get_number_2020(input: &[usize]) -> Result<usize, String> {
    if input.is_empty() {
        return Err("No initial numbers".to_owned());
    }
    let mut seen: HashMap<usize, usize> = input[0..input.len() - 1]
        .iter()
        .enumerate()
        .map(|(i, n)| (*n, i))
        .collect();
    let mut prev = input[input.len() - 1];

    for turn in input.len()..2020 {
        let current: usize = seen.get(&prev).map(|t| turn - t - 1).unwrap_or(0);
        seen.insert(prev, turn - 1);
        prev = current;
    }
    Ok(prev)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_number_2020_works_for_example() {
        // given
        let input = &[0, 3, 6];

        // when
        let result = get_number_2020(input).expect("Expected no failure");

        // then
        assert_eq!(result, 436);
    }
}
