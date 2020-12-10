use std::cmp::min;
use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(&Path::new(&filename)).map_err(|e| e.to_string())?;
    let adapters = parse_input(&content)?;

    if let Some(adapter_chain_diffs) = solve_adapter_chain(&adapters) {
        println!("Adapter chain diff product: {}", adapter_chain_diffs);
    } else {
        println!("no valid adapter chain");
    }

    let possible_adapter_combs = count_adapter_combinations(&adapters);
    println!(
        "There are {} possible ways to connect your device.",
        possible_adapter_combs
    );

    Ok(())
}

fn count_adapter_combinations(sorted_adapters: &[u64]) -> u64 {
    if sorted_adapters.is_empty() {
        return 0;
    }
    let mut combinations = Vec::with_capacity(sorted_adapters.len());
    combinations.push(1);
    for i in 1..sorted_adapters.len() {
        let mut count = 0;
        for j in (i - min(i, 3))..i {
            if sorted_adapters[i] - sorted_adapters[j] < 4 {
                count += combinations[j];
            }
        }
        combinations.push(count);
    }
    combinations[combinations.len() - 1]
}

fn solve_adapter_chain(sorted_adapters: &[u64]) -> Option<usize> {
    let differences: Vec<u64> = sorted_adapters.windows(2).map(|w| w[1] - w[0]).collect();

    if differences.iter().any(|d| *d > 3) {
        return None;
    }
    let one_differences = differences.iter().filter(|d| **d == 1).count();
    let three_differences = differences.iter().filter(|d| **d == 3).count();

    Some(one_differences * three_differences)
}

fn parse_input(content: &str) -> Result<Vec<u64>, String> {
    let mut adapters = content
        .split_terminator('\n')
        .map(|s| {
            s.parse()
                .map_err(|e| format!("Unable to parse joltage '{}'; {}", s, e))
        })
        .collect::<Result<Vec<u64>, String>>()?;

    let max_adapter = *adapters
        .iter()
        .max()
        .ok_or_else(|| "Empty input".to_owned())?;
    adapters.push(0);
    adapters.push(max_adapter + 3);
    adapters.sort_unstable();

    Ok(adapters)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_adapter_chain_works_for_example_1() {
        // given
        let adapters =
            parse_input("16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4\n").expect("Expected valid input");

        // when
        let result = solve_adapter_chain(&adapters);

        // then
        assert_eq!(result, Some(35));
    }

    #[test]
    fn solve_adapter_chain_works_for_example_2() {
        // given
        let adapters = parse_input(
            "28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3\n").expect("Expected valid input");

        // when
        let result = solve_adapter_chain(&adapters);

        // then
        assert_eq!(result, Some(220));
    }

    #[test]
    fn count_adapter_combinations_works_for_example_1() {
        // given
        let adapters =
            parse_input("16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4\n").expect("Expected valid input");

        // when
        let result = count_adapter_combinations(&adapters);

        // then
        assert_eq!(result, 8);
    }

    #[test]
    fn count_adapter_combinations_works_for_example_2() {
        // given
        let adapters = parse_input(
            "28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3\n").expect("Expected valid input");

        // when
        let result = count_adapter_combinations(&adapters);

        // then
        assert_eq!(result, 19208);
    }
}
