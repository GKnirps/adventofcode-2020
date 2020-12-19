use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(&Path::new(&filename)).map_err(|e| e.to_string())?;

    let (rules, messages) = parse_input(&content)?;
    let rules_cnf = rules_to_cnf(&rules);
    let inverted_rules = inverse_rules(&rules_cnf);

    let valid_messages = messages
        .iter()
        .filter(|message| cyk(&inverted_rules, message))
        .count();
    println!("You have {} new valid messages.", valid_messages);

    let modified_rules = modify_rules(rules)?;
    let modified_rules_cnf = rules_to_cnf(&modified_rules);
    let inverted_mod_rules = inverse_rules(&modified_rules_cnf);
    let valid_messages_with_modified_rules = messages
        .iter()
        .filter(|message| cyk(&inverted_mod_rules, message))
        .count();
    println!(
        "You have {} new messages that are valid according to the modified rules",
        valid_messages_with_modified_rules
    );

    Ok(())
}

fn modify_rules(mut rules: Vec<Rule>) -> Result<Vec<Rule>, String> {
    // sanity check first
    if rules.len() < 12 {
        return Err(format!(
            "Unable to modify rule #11, there are only {} rules",
            rules.len()
        ));
    }
    if rules[8] != Rule::Sub(vec![RuleSubst::Mono(42)])
        || rules[11] != Rule::Sub(vec![RuleSubst::Cat(42, 31)])
    {
        return Err("Rules to modify do not match expectations".to_owned());
    }

    rules[8] = Rule::Sub(vec![RuleSubst::Mono(42), RuleSubst::Cat(42, 8)]);
    // our rules_to_cnf function does not handle splitting longer rules, but for this one case
    // we can handle this here manually
    rules[11] = Rule::Sub(vec![
        RuleSubst::Cat(42, 31),
        RuleSubst::Cat(42, rules.len()),
    ]);
    rules.push(Rule::Sub(vec![RuleSubst::Cat(11, 31)]));

    Ok(rules)
}

// Yes, I know that the problem is regular
// but the rules are already _almost_ in chomsky normal form and far from being a regular grammar
// and even with a regular grammar I had to build a deterministic nfm…
// TL;DR I am to lazy to solve this efficiently
fn cyk(rules: &HashMap<RuleCnfLhs, Vec<usize>>, word: &str) -> bool {
    let word_length = word.chars().count();
    let mut v: Vec<HashSet<usize>> = (0..(word_length * word_length))
        .map(|_| HashSet::new())
        .collect();
    for (i, c) in word.chars().enumerate() {
        if let Some(rule_ids) = rules.get(&RuleCnfLhs::Lit(c)) {
            for rule_id in rule_ids {
                v[i].insert(*rule_id);
            }
        }
    }
    // this is the second-slowest implementation of the cyk algorithm I've ever seen
    // and screw wikipedia for its 1-based fucking indexing!
    for j in 1..word_length {
        for i in 0..(word_length - j) {
            for k in 0..j {
                for rule_id in get_rules_matching(
                    rules,
                    &v[i + k * word_length],
                    &v[i + k + 1 + (j - k - 1) * word_length],
                ) {
                    v[j * word_length + i].insert(rule_id);
                }
            }
        }
    }

    v[word_length * (word_length - 1)].contains(&0)
}

fn get_rules_matching(
    rules: &HashMap<RuleCnfLhs, Vec<usize>>,
    vleft: &HashSet<usize>,
    vright: &HashSet<usize>,
) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::with_capacity(rules.len());
    for b in vleft {
        for c in vright {
            if let Some(rule_ids) = rules.get(&RuleCnfLhs::Sub(*b, *c)) {
                for rule_id in rule_ids {
                    result.push(*rule_id);
                }
            }
        }
    }
    result
}

fn rules_to_cnf(in_rules: &[Rule]) -> Vec<Vec<RuleCnfLhs>> {
    in_rules
        .iter()
        .map(|rule| rule_to_cnf(rule, in_rules))
        .collect()
}

fn rule_to_cnf(rule: &Rule, in_rules: &[Rule]) -> Vec<RuleCnfLhs> {
    match rule {
        Rule::Lit(c) => vec![RuleCnfLhs::Lit(*c)],
        Rule::Sub(substitutions) => substitutions
            .iter()
            .map(|subst| match subst {
                RuleSubst::Cat(a, b) => vec![RuleCnfLhs::Sub(*a, *b)],
                RuleSubst::Mono(a) => rule_to_cnf(&in_rules[*a], in_rules),
            })
            .flatten()
            .collect(),
    }
}

fn inverse_rules(rules: &[Vec<RuleCnfLhs>]) -> HashMap<RuleCnfLhs, Vec<usize>> {
    rules.iter().enumerate().fold(
        HashMap::with_capacity(rules.len() * 2),
        |mut inverted, (rule_id, rule)| {
            for lhs in rule {
                inverted
                    .entry(*lhs)
                    .or_insert_with(|| Vec::with_capacity(rules.len()))
                    .push(rule_id);
            }
            inverted
        },
    )
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
enum RuleCnfLhs {
    Lit(char),
    Sub(usize, usize),
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

    #[test]
    fn cyk_works_for_examples() {
        // given
        let (rules, words) = parse_input(
            r#"0: 4 6
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"
6: 1 5

ababbb
bababa
abbbab
aaabbb
aaaabbb"#,
        )
        .expect("Expected example input to parse");
        let rules_cnf = rules_to_cnf(&rules);
        let inv_rules = inverse_rules(&rules_cnf);

        // when
        let results: Vec<bool> = words.iter().map(|word| cyk(&inv_rules, word)).collect();

        // then
        assert_eq!(&results, &[true, false, true, false, false]);
    }
}
