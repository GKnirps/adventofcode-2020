use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(&Path::new(&filename)).map_err(|e| e.to_string())?;

    let groups = parse_groups_any(&content);

    let any_yes_answers = count_yes_answers(&groups);
    println!(
        "Sum of the any-yes-answers of all groups: {}",
        any_yes_answers
    );

    let all_yes_answers = count_yes_answers(&parse_groups_all(&content));
    println!("Sum of all-yes-answers of all groups: {}", all_yes_answers);

    Ok(())
}

fn count_yes_answers(groups: &[[bool; 26]]) -> usize {
    groups
        .iter()
        .map(|group| group.iter().filter(|a| **a).count())
        .sum()
}

fn parse_groups_any(content: &str) -> Vec<[bool; 26]> {
    content
        .split("\n\n")
        .filter(|g| !g.is_empty())
        .map(parse_group_any)
        .collect()
}

fn parse_group_any(input: &str) -> [bool; 26] {
    let mut answers = [false; 26];
    for c in input
        .chars()
        .filter(|c| c.is_ascii_alphabetic() && c.is_ascii_lowercase())
    {
        let mut buf = [0; 1];
        c.encode_utf8(&mut buf);
        answers[(buf[0] - 97) as usize] = true;
    }
    answers
}

fn parse_groups_all(content: &str) -> Vec<[bool; 26]> {
    content
        .split("\n\n")
        .filter(|g| !g.is_empty())
        .map(parse_group_all)
        .collect()
}

fn parse_group_all(input: &str) -> [bool; 26] {
    input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(parse_group_any)
        .fold([true; 26], |mut all, this| {
            for i in 0..26 {
                all[i] = all[i] && this[i]
            }
            all
        })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn count_any_yes_answers_works_correctly() {
        // given
        let input = r"abc

a
b
c

ab
ac

a
a
a
a

b";

        // when
        let groups = parse_groups_any(input);
        let count = count_yes_answers(&groups);

        // then
        assert_eq!(count, 11);
    }

    #[test]
    fn count_all_yes_answers_works_correctly() {
        // given
        let input = r"abc

a
b
c

ab
ac

a
a
a
a

b";

        // when
        let groups = parse_groups_all(input);
        let count = count_yes_answers(&groups);

        // then
        assert_eq!(count, 6);
    }
}
