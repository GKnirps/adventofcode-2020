use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(&Path::new(&filename)).map_err(|e| e.to_string())?;
    let lines: Vec<&str> = content.split('\n').filter(|l| !l.is_empty()).collect();
    let passwords: Vec<Password> = lines
        .iter()
        .map(|l| parse_line(l))
        .collect::<Result<Vec<Password>, String>>()?;

    let n_valid_passwords = passwords.iter().filter(|pw| check_pw(pw)).count();
    let new_n_valid_passwords = passwords.iter().filter(|pw| check_pw_new(pw)).count();

    println!("There are {} valid passwords", n_valid_passwords);
    println!(
        "There are {} valid passwords for the actual rules",
        new_n_valid_passwords
    );

    Ok(())
}
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
struct Password<'a> {
    min: usize,
    max: usize,
    letter: char,
    pwd: &'a str,
}

fn check_pw(pw: &Password) -> bool {
    let count = pw.pwd.chars().filter(|c| *c == pw.letter).count();
    count >= pw.min && count <= pw.max
}

fn check_pw_new(pw: &Password) -> bool {
    // indices start with 1
    if pw.min == 0 || pw.max == 0 {
        return false;
    }
    let mut count = 0;
    let pwd = pw.pwd;
    let mut chars = pwd.chars().skip(&pw.min - 1);
    let char1 = chars.next();
    if char1 == Some(pw.letter) {
        count += 1;
    }
    let mut chars = chars.skip(pw.max - pw.min - 1);
    let char2 = chars.next();
    if char2 == Some(pw.letter) {
        count += 1;
    }

    count == 1
}

fn parse_line(line: &str) -> Result<Password, String> {
    // I dont want to add external dependencies. Screw you, regular expressions!
    let mut first_splitter = line.splitn(2, ' ');
    let numbers = first_splitter
        .next()
        .ok_or_else(|| format!("line {} contains nothing", line))?;
    let rest = first_splitter
        .next()
        .ok_or_else(|| format!("line {} does not contain any whitespaces", line))?;

    let mut numbers_split = numbers.splitn(2, '-');
    let min: usize = numbers_split
        .next()
        .ok_or_else(|| format!("Invalid format for min/max in line {}", line))?
        .parse::<usize>()
        .map_err(|e| format!("Unable to parse min in line {}: {}", line, e))?;
    let max: usize = numbers_split
        .next()
        .ok_or_else(|| format!("Invalid format for min/max in line {}", line))?
        .parse::<usize>()
        .map_err(|e| format!("Unable to parse min in line {}: {}", line, e))?;

    let mut rest_split = rest.splitn(2, ':');
    let letter = rest_split
        .next()
        .ok_or_else(|| format!("line {} has no colon", line))?
        .chars()
        .next()
        .ok_or_else(|| format!("line {} has not letter before the colon", line))?;
    let pwd = rest_split
        .next()
        .ok_or_else(|| format!("line {} does not contain a password", line))?
        .trim();

    Ok(Password {
        min,
        max,
        letter,
        pwd,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_line_parses_valid_line() {
        // given
        let line = "4-11 g: abcdef";

        // when
        let pr = parse_line(line);

        // then
        let p = pr.expect("Expected successful parsing");
        assert_eq!(
            p,
            Password {
                min: 4,
                max: 11,
                letter: 'g',
                pwd: "abcdef"
            }
        );
    }

    #[test]
    fn check_pw_new_should_accept_valid_passwords() {
        // given
        let valid = Password {
            min: 1,
            max: 3,
            letter: 'a',
            pwd: "abcde",
        };

        // when
        let result = check_pw_new(&valid);

        // then
        assert!(result);
    }

    #[test]
    fn check_pw_new_should_reject_invalid_passwords_with_no_occurences() {
        // given
        let password = Password {
            min: 1,
            max: 3,
            letter: 'b',
            pwd: "cdefg",
        };

        // when
        let result = check_pw_new(&password);

        // then
        assert!(!result);
    }

    #[test]
    fn check_pw_new_should_reject_invalid_passwords_with_two_occurences() {
        // given
        let password = Password {
            min: 2,
            max: 9,
            letter: 'c',
            pwd: "ccccccccc",
        };

        // when
        let result = check_pw_new(&password);

        // then
        assert!(!result);
    }
}
