use std::collections::HashMap;

fn main() -> Result<(), String> {
    let input: &[usize] = &[14, 3, 1, 0, 9, 5];

    let number_2020 = get_number_at_turn(2020, input)?;
    println!("Number at turn #2020 is: {}", number_2020);

    // After yesterday, I need a day off. So I'm bruteforcing this
    // Takes 3.5s and some dozen MiB of memory, so who cares?
    // 2.8s if I reserve some more memory up front
    // Hey, If they did not want me to bute force it, they would have picked a higher number.
    let number_30_m = get_number_at_turn(30_000_000, input)?;
    println!("Number at turn #30000000 is: {}", number_30_m);

    Ok(())
}

fn get_number_at_turn(final_turn: usize, input: &[usize]) -> Result<usize, String> {
    if input.is_empty() {
        return Err("No initial numbers".to_owned());
    }
    let mut seen: HashMap<usize, usize> = input[0..input.len() - 1]
        .iter()
        .enumerate()
        .map(|(i, n)| (*n, i))
        .collect();
    seen.reserve(input.len() / 4);
    let mut prev = input[input.len() - 1];

    for turn in input.len()..final_turn {
        let current: usize = seen.get(&prev).map(|t| turn - t - 1).unwrap_or(0);
        seen.insert(prev, turn - 1);
        prev = current;
    }
    println!(
        "seen {} numbers, {} MiB",
        seen.len(),
        seen.len() * 8 / 1024 / 1024
    );
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
        let result = get_number_at_turn(2020, input).expect("Expected no failure");

        // then
        assert_eq!(result, 436);
    }
}
