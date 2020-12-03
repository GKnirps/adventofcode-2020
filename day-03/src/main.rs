use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(&Path::new(&filename)).map_err(|e| e.to_string())?;
    let map = parse_input(&content);

    let trees_on_path = count_trees_on_path(&map);

    println!("There are {} trees on the path.", trees_on_path);

    Ok(())
}

fn count_trees_on_path(map: &Map) -> usize {
    map.rows
        .iter()
        .enumerate()
        .filter(|(column, row_content)| row_content[(column * 3) % row_content.len()])
        .count()
}

struct Map {
    rows: Vec<Vec<bool>>,
}

fn parse_input(input: &str) -> Map {
    let rows = input
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(|c| c == '#').collect())
        .collect();
    Map { rows }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        // given
        let input = "..#.\n###.\n...#";

        // when
        let result = parse_input(input);

        // then
        assert_eq!(
            &result.rows,
            &[
                vec!(false, false, true, false),
                vec!(true, true, true, false),
                vec!(false, false, false, true)
            ]
        );
    }
}
