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

fn inverse_graph<'a>(graph: &'a HashMap<&'a str, Vec<&'a str>>) -> HashMap<&'a str, Vec<&'a str>> {
    let mut inverted: HashMap<&'a str, Vec<&'a str>> = HashMap::with_capacity(graph.len());

    for (lhs, rhs) in graph.iter() {
        for key in rhs {
            inverted
                .entry(key)
                .or_insert_with(|| Vec::with_capacity(graph.len()))
                .push(lhs);
        }
    }

    inverted
}

fn parse_lines(content: &str) -> Result<HashMap<&str, Vec<&str>>, String> {
    content
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(parse_line)
        .collect()
}

fn parse_line(line: &str) -> Result<(&str, Vec<&str>), String> {
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

    let rhs: Vec<&str> = rhs_raw
        .split(", ")
        .map(|s| {
            s.trim()
                .trim_end_matches('.')
                .trim_end_matches("bags")
                .trim_end_matches("bag")
        })
        // we don't care about the amount (yet). maybe it is required for part 2
        .map(|s| s.trim_start_matches(|c: char| c.is_ascii_digit()).trim())
        .collect();

    Ok((lhs, rhs))
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
            &["dark maroon", "dim maroon", "striped green", "pale aqua"]
        );
    }
}
