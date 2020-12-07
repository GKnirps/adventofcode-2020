use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(&Path::new(&filename)).map_err(|e| e.to_string())?;

    let graph = parse_lines(&content)?;

    let inversed_graph = inverse_graph(&graph);
    let descendants = count_descendants(&inversed_graph, "shiny gold");
    println!(
        "{} bag colors can eventually contain at least one shiny gold bag",
        descendants
    );

    Ok(())
}

fn count_descendants(graph: &HashMap<&str, Vec<&str>>, start: &str) -> usize {
    let mut seen: HashSet<&str> = HashSet::with_capacity(graph.len());
    let mut stack: Vec<&str> = Vec::with_capacity(graph.len());
    stack.push(start);

    while let Some(v) = stack.pop() {
        seen.insert(v);
        if let Some(children) = graph.get(v) {
            for child in children {
                if !seen.contains(child) {
                    stack.push(child);
                }
            }
        }
    }

    seen.len() - 1
}

fn inverse_graph<'a>(
    graph: &'a HashMap<&'a str, Vec<(&'a str, u64)>>,
) -> HashMap<&'a str, Vec<&'a str>> {
    let mut inverted: HashMap<&'a str, Vec<&'a str>> = HashMap::with_capacity(graph.len());

    for (lhs, rhs) in graph.iter() {
        for (key, _) in rhs {
            inverted
                .entry(key)
                .or_insert_with(|| Vec::with_capacity(graph.len()))
                .push(lhs);
        }
    }

    inverted
}

fn parse_lines(content: &str) -> Result<HashMap<&str, Vec<(&str, u64)>>, String> {
    content
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(parse_line)
        .collect()
}

fn parse_line(line: &str) -> Result<(&str, Vec<(&str, u64)>), String> {
    let mut split1 = line.splitn(2, " bags contain ");
    let lhs = split1
        .next()
        .filter(|s| !s.is_empty())
        .ok_or_else(|| format!("line '{}' is invalid", line))?;

    let rhs_raw = split1
        .next()
        .filter(|s| !s.is_empty())
        .ok_or_else(|| format!("line '{}' has no valid right hand side", line))?;

    if rhs_raw == "no other bags." {
        return Ok((lhs, vec![]));
    }

    let rhs: Result<Vec<(&str, u64)>, String> = rhs_raw
        .split(", ")
        .map(|s| parse_num_and_color(s, line))
        .collect();

    Ok((lhs, rhs?))
}

fn parse_num_and_color<'a>(s: &'a str, full_line: &str) -> Result<(&'a str, u64), String> {
    let mut num_color_split = s
        .trim()
        .trim_end_matches('.')
        .trim_end_matches("bags")
        .trim_end_matches("bag")
        .splitn(2, ' ');
    let num = num_color_split
        .next()
        .ok_or_else(|| {
            format!(
                "Line '{}' does not have an amount of bags for all contained bags",
                full_line
            )
        })
        .and_then(|s| {
            s.parse::<u64>()
                .map_err(|e| format!("Invalid amount of bags in line '{}': {}", full_line, e))
        })?;
    let color = num_color_split
        .next()
        .ok_or_else(|| format!("Missing color in line '{}'", full_line))?
        .trim()
        .trim_end_matches('.');
    Ok((color, num))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_line_parses_valid_line() {
        // given
        let line = "light orange bags contain 1 dark maroon bag, 3 dim maroon bags, 5 striped green bags, 2 pale aqua bags.";

        // when
        let result = parse_line(line);

        // then
        let (lhs, rhs) = result.expect("Expected valid result");
        assert_eq!(lhs, "light orange");
        assert_eq!(
            &rhs,
            &[
                ("dark maroon", 1),
                ("dim maroon", 3),
                ("striped green", 5),
                ("pale aqua", 2)
            ]
        );
    }
}
