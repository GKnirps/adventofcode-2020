use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(&Path::new(&filename)).map_err(|e| e.to_string())?;
    let (rules, _, nearby_tickets) = parse_input(&content)?;

    let error_sum = ticket_scanning_error_sum(&rules, &nearby_tickets);
    println!("The ticket scanning error sum is {}", error_sum);

    Ok(())
}

fn ticket_scanning_error_sum(rules: &[Rule], tickets: &[Vec<u64>]) -> u64 {
    tickets
        .iter()
        .flatten()
        .filter(|value| !rules.iter().any(|rule| value_matches_rule(rule, **value)))
        .sum()
}

fn value_matches_rule(rule: &Rule, value: u64) -> bool {
    rule.ranges
        .iter()
        .any(|(from, to)| value >= *from && value <= *to)
}

fn parse_input(content: &str) -> Result<(Vec<Rule>, Vec<u64>, Vec<Vec<u64>>), String> {
    let mut split = content.splitn(3, "\n\n");
    let rules = parse_rules(split.next().expect("Expected rules"))?;
    let own_ticket = split
        .next()
        .ok_or_else(|| "Input is missing own ticket".to_owned())
        .and_then(|s| parse_ticket(s.trim_start_matches("your ticket:\n")))?;
    let nearby_tickets = split
        .next()
        .ok_or_else(|| "Input is missing nearby tickets".to_owned())
        .and_then(|s| parse_tickets(s.trim_start_matches("nearby tickets:\n")))?;

    Ok((rules, own_ticket, nearby_tickets))
}

fn parse_tickets(lines: &str) -> Result<Vec<Vec<u64>>, String> {
    lines.split_terminator('\n').map(parse_ticket).collect()
}

fn parse_ticket(line: &str) -> Result<Vec<u64>, String> {
    line.split(',')
        .map(|s| {
            s.parse::<u64>()
                .map_err(|e| format!("Unable to parse ticket '{}': {}", line, e))
        })
        .collect()
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
struct Rule<'a> {
    field_name: &'a str,
    ranges: [(u64, u64); 2],
}

fn parse_rules(lines: &str) -> Result<Vec<Rule>, String> {
    lines.split_terminator('\n').map(parse_rule).collect()
}

fn parse_rule(line: &str) -> Result<Rule, String> {
    let mut split = line.splitn(2, ": ");
    let field_name = split.next().expect("Expected field name");
    let ranges = parse_ranges(
        split
            .next()
            .ok_or_else(|| format!("no ranges in line {}", line))?,
    )?;
    Ok(Rule { field_name, ranges })
}

fn parse_ranges(input: &str) -> Result<[(u64, u64); 2], String> {
    let mut split = input.splitn(2, " or ");
    let first = parse_range(split.next().expect("Expected first range"))?;
    let second = parse_range(
        split
            .next()
            .ok_or_else(|| format!("Expected second range in line {}", input))?,
    )?;

    Ok([first, second])
}

fn parse_range(input: &str) -> Result<(u64, u64), String> {
    let mut split = input.splitn(2, '-');
    let from = split
        .next()
        .expect("Expected first value in range")
        .parse::<u64>()
        .map_err(|e| format!("Unable to parse first value in range'{}': {}", input, e))?;
    let to = split
        .next()
        .ok_or_else(|| format!("Expected second value in range '{}'", input))?
        .parse::<u64>()
        .map_err(|e| format!("Unable to parse second value in range'{}': {}", input, e))?;
    Ok((from, to))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ticket_scanning_error_sum_works_for_example() {
        // given
        let input = r"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
        let (rules, _, nearby_tickets) = parse_input(input).expect("Expected valid example input");

        // when
        let sum = ticket_scanning_error_sum(&rules, &nearby_tickets);

        // then
        assert_eq!(sum, 71);
    }
}
