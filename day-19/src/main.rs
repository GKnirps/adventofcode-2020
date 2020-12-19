use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(&Path::new(&filename)).map_err(|e| e.to_string())?;

    let (rules, messages) = parse_input(&content)?;

    Ok(())
}

fn parse_input(content: &str) -> Result<(Vec<Rule>, Vec<&str>), String> {
    let mut split = content.splitn(2, "\n\n");
    let rules_with_ids = split
        .next()
        .expect("Expected rules at the beginning")
        .split('\n')
        .map(parse_rule)
        .collect::<Result<Vec<(usize, Rule)>, String>>()?;
    let rules = rules_indexed_by_id(rules_with_ids)?;
    let messages = split
        .next()
        .ok_or_else(|| "Expected messages after rules".to_owned())?
        .split_terminator('\n')
        .collect::<Vec<&str>>();

    Ok((rules, messages))
}

fn rules_indexed_by_id(mut rules_with_ids: Vec<(usize, Rule)>) -> Result<Vec<Rule>, String> {
    rules_with_ids.sort_unstable_by_key(|(id, _)| *id);

    rules_with_ids
        .into_iter()
        .enumerate()
        .map(|(expected_id, (actual_id, rule))| {
            if expected_id != actual_id {
                Err(format!(
                    "Rule IDs are not an unbroken sequence, expected {} but got {}",
                    expected_id, actual_id
                ))
            } else {
                Ok(rule)
            }
        })
        .collect()
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
enum Rule {
    Lit(char),
    Sub(Vec<RuleSubst>),
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
enum RuleSubst {
    Mono(usize),
    Cat(usize, usize),
}

fn parse_rule(line: &str) -> Result<(usize, Rule), String> {
    let mut split = line.splitn(2, ": ");
    let id = split
        .next()
        .expect("Expected rule ID")
        .parse::<usize>()
        .map_err(|e| format!("Invalid rule ID in line '{}': {}", line, e))?;

    let lhs = split
        .next()
        .ok_or_else(|| format!("Expected left hand side of rule in line '{}'", line))?
        .trim();

    if lhs.starts_with('"') {
        let c = lhs
            .chars()
            .nth(1)
            .ok_or_else(|| format!("Expected literal rule in line '{}'", line))?;
        Ok((id, Rule::Lit(c)))
    } else if let Ok(sub_id) = lhs.parse::<usize>() {
        Ok((id, Rule::Sub(vec![RuleSubst::Mono(sub_id)])))
    } else {
        let options: Vec<RuleSubst> = lhs
            .split(" | ")
            .map(|ids_str| {
                let mut ids = ids_str.splitn(2, ' ').map(|id| {
                    id.parse::<usize>().map_err(|e| {
                        format!(
                            "Unable to parse ID '{}' in lhs of line '{}': {}",
                            id, line, e
                        )
                    })
                });
                let first = ids.next().expect("Expected rule ID")?;
                Ok(match ids.next().transpose()? {
                    Some(second) => RuleSubst::Cat(first, second),
                    None => RuleSubst::Mono(first),
                })
            })
            .collect::<Result<Vec<RuleSubst>, String>>()?;
        Ok((id, Rule::Sub(options)))
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
