use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(&Path::new(&filename)).map_err(|e| e.to_string())?;
    let map = parse_input(&content);

    let trees_on_path = count_trees_on_path(&map, 1, 3);

    println!(
        "There are {} trees on the path with a 3 right, 1 down slope",
        trees_on_path
    );

    let solution_2 = solve_part_2(&map);
    println!(
        "The product of the number of trees on the different paths is {}",
        solution_2
    );

    Ok(())
}

fn count_trees_on_path(map: &Map, row_step: usize, col_step: usize) -> usize {
    let mut current_row = 0;
    let mut current_col = 0;
    let mut tree_count = 0;

    while current_row < map.rows.len() {
        tree_count += if map.get(current_row, current_col) {
            1
        } else {
            0
        };
        current_row += row_step;
        current_col += col_step;
    }

    tree_count
}

fn solve_part_2(map: &Map) -> usize {
    (&[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)])
        .iter()
        .map(|(col_step, row_step)| count_trees_on_path(&map, *row_step, *col_step))
        .product()
}

struct Map {
    rows: Vec<Vec<bool>>,
}

impl Map {
    fn get(&self, row: usize, column: usize) -> bool {
        let rows = &self.rows;
        if row >= rows.len() {
            return false;
        }
        let row_length = rows[row].len();
        rows[row][column % row_length]
    }
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

    #[test]
    fn test_solution_2() {
        // given
        let map = parse_input(
            r"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#",
        );

        // when
        let result = solve_part_2(&map);

        // then
        assert_eq!(result, 336);
    }
}
