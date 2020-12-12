use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(&Path::new(&filename)).map_err(|e| e.to_string())?;

    let actions = parse_actions(&content)?;
    let (north, east) = run_instructions(&actions);
    println!(
        "Manhattan distance to the origin: {}",
        north.abs() + east.abs()
    );

    Ok(())
}

fn run_instructions(actions: &[Action]) -> (i64, i64) {
    let mut north = 0;
    let mut east = 0;
    let mut dir = 0;

    for action in actions {
        match action {
            Action::Ver(v) => {
                north += v;
            }
            Action::Hor(v) => {
                east += v;
            }
            Action::Rot(v) => {
                dir = (dir + v).rem_euclid(4);
            }
            Action::Forward(v) => match dir.rem_euclid(4) {
                0 => {
                    east += v;
                }
                1 => {
                    north += v;
                }
                2 => {
                    east -= v;
                }
                3 => {
                    north -= v;
                }
                _ => panic!("Unknown direction"),
            },
        }
    }

    (north, east)
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
enum Action {
    Hor(i64),
    Ver(i64),
    Forward(i64),
    Rot(i64),
}

fn parse_actions(content: &str) -> Result<Vec<Action>, String> {
    content.split_terminator('\n').map(parse_action).collect()
}

fn parse_action(input: &str) -> Result<Action, String> {
    let action_short = input
        .chars()
        .next()
        .ok_or_else(|| "Unable to parse action: Empty input".to_owned())?;
    if !action_short.is_ascii() {
        return Err(format!("Action '{}' is not ascii", action_short));
    }
    let value: i64 = input
        .split_at(1)
        .1
        .parse()
        .map_err(|e| format!("Invalid value for action {}: {}", input, e))?;

    Ok(match action_short {
        'N' => Action::Ver(value),
        'S' => Action::Ver(-value),
        'E' => Action::Hor(value),
        'W' => Action::Hor(-value),
        'L' => {
            if value % 90 != 0 {
                return Err(format!("Rotation in {} is not a multiple of 90", input));
            }
            Action::Rot(value / 90)
        }
        'R' => {
            if value % 90 != 0 {
                return Err(format!("Rotation in {} is not a multiple of 90", input));
            }
            Action::Rot(-(value / 90))
        }
        'F' => Action::Forward(value),
        _ => {
            return Err(format!("Unknown action {} in {}", action_short, input));
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;
}
