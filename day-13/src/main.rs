use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(&Path::new(&filename)).map_err(|e| e.to_string())?;

    let (earliest_departure, bus_ids) = parse_without(&content)?;
    let (next_bus_id, next_bus_timestamp) =
        find_earliest_bus_after_timestamp(earliest_departure, &bus_ids)
            .ok_or_else(|| "No busses arrive at all!".to_owned())?;
    println!(
        "The next bus id times the waiting time is {}",
        next_bus_id * (next_bus_timestamp - earliest_departure)
    );

    let earliest_departure_sequence = find_earliest_departure_sequence(&bus_ids)
        .ok_or_else(|| "Unable to find departure sequence".to_owned())?;
    println!(
        "The earliest sequence where busses arrive as described is at {}",
        earliest_departure_sequence
    );

    Ok(())
}

fn find_earliest_bus_after_timestamp(
    timestamp: u64,
    bus_ids: &[Option<u64>],
) -> Option<(u64, u64)> {
    bus_ids
        .iter()
        .filter_map(|id| *id)
        .map(|id| (id, (timestamp / id + 1) * id))
        .min_by_key(|(_, ts)| *ts)
}

fn find_earliest_departure_sequence(bus_ids: &[Option<u64>]) -> Option<i64> {
    // we assume all bus ids are prime numbers
    // if that is not the case we would need to modify this approach a bit
    if !all_prime(bus_ids) {
        return None;
    }

    let bus_ids_with_offset: Vec<(i64, i64)> = bus_ids
        .iter()
        .enumerate()
        .filter_map(|(i, opt_id)| opt_id.map(|id| (id as i64, -(i as i64))))
        .collect();

    // all ids are prime (see above) => all ids are coprime => the lcm ist the product of the ids
    let lcm: i64 = bus_ids_with_offset.iter().map(|(id, _)| id).product();

    let x = bus_ids_with_offset
        .iter()
        .map(|(m, a)| {
            let (_, _, s) = gcd(*m, lcm / m);
            a * s * (lcm / m)
        })
        .sum::<i64>();
    //Some((x % lcm  + lcm) % lcm)
    Some(x.rem_euclid(lcm))
}

fn gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a, 1, 0);
    }
    let (d, s, t) = gcd(b, a % b);
    (d, t, s - (a / b) * t)
}

fn all_prime(numbers: &[Option<u64>]) -> bool {
    let max = numbers.iter().filter_map(|i| *i).max().unwrap_or(0) as usize;

    if max < 2 {
        return false;
    }

    let mut sieve = vec![false; max + 1];
    sieve[0] = true;
    sieve[1] = true;

    for k in 2..((max as f64).sqrt() as usize + 1) {
        if !sieve[k] {
            let mut l = k * k;
            while l < sieve.len() {
                sieve[l] = true;
                l += k;
            }
        }
    }

    numbers
        .iter()
        .filter_map(|i| *i)
        .all(|k| !sieve[k as usize])
}

fn parse_without(input: &str) -> Result<(u64, Vec<Option<u64>>), String> {
    let mut lines = input.split_terminator('\n');
    let earliest_departure: u64 = lines
        .next()
        .expect("Expected at least one line")
        .parse()
        .map_err(|e| format!("Unable to parse earliest departure timestamp: {}", e))?;
    let bus_ids_string = lines
        .next()
        .ok_or_else(|| "No bus IDs in input".to_owned())?;
    let bus_ids = parse_bus_ids(bus_ids_string)?;

    Ok((earliest_departure, bus_ids))
}

fn parse_bus_ids(line: &str) -> Result<Vec<Option<u64>>, String> {
    line.split(',')
        .map(|s| {
            if s == "x" {
                Ok(None)
            } else {
                s.parse::<u64>()
                    .map(Some)
                    .map_err(|e| format!("Unable to parse bus id {}: {}", s, e))
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn find_earliest_bus_after_timestamp_works_for_example() {
        // given
        let (earliest_departure, bus_ids) =
            parse_without("939\n7,13,x,x,59,x,31,19\n").expect("Expected valid example input");

        // when
        let result = find_earliest_bus_after_timestamp(earliest_departure, &bus_ids);

        // then
        assert_eq!(result, Some((59, 944)));
    }

    #[test]
    fn find_earliest_departure_sequence_works_for_example() {
        // given
        let (_, bus_ids) =
            parse_without("939\n7,13,x,x,59,x,31,19\n").expect("Expected valid example input");

        // when
        let result = find_earliest_departure_sequence(&bus_ids);

        // then
        assert_eq!(result, Some(1068781));
    }

    #[test]
    fn all_prime_checks_if_all_present_inputs_are_prime() {
        // given
        let input = &[Some(2), Some(11), None, Some(3)];

        // when
        let result = all_prime(input);

        // then
        assert!(result);
    }

    #[test]
    fn all_prime_detects_non_prime_inputs() {
        // given
        let input = &[Some(2), Some(11), None, Some(3), Some(42)];

        // when
        let result = all_prime(input);

        // then
        assert!(!result);
    }

    #[test]
    fn gcd_returns_correct_values() {
        assert_eq!(gcd(8, 12), (4, -1, 1));
        assert_eq!(gcd(4, 12), (4, 1, 0));
        assert_eq!(gcd(3, 13), (1, -4, 1));
    }
}
