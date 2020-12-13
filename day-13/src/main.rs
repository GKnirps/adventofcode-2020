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
}
