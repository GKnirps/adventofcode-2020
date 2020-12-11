use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(&Path::new(&filename)).map_err(|e| e.to_string())?;

    let ferry = parse_input(&content)?;

    let occupied_seats = seats_at_equilibrium(ferry);
    println!("In a stable state, {} seats are occupied", occupied_seats);

    Ok(())
}

// I don't know if this terminates for all inputs.
fn seats_at_equilibrium(mut ferry: Ferry) -> usize {
    loop {
        let next = next_gen(&ferry);
        if ferry == next {
            return next.cells.iter().filter(|c| **c == Cell::Occupied).count();
        }
        ferry = next;
    }
}

fn next_gen(ferry: &Ferry) -> Ferry {
    let cells: Vec<Cell> = ferry
        .cells
        .iter()
        .enumerate()
        .map(|(i, cell)| match cell {
            Cell::Floor => Cell::Floor,
            Cell::Occupied => {
                if count_occupied_seats_around(ferry, i) > 3 {
                    Cell::Seat
                } else {
                    Cell::Occupied
                }
            }
            Cell::Seat => {
                if count_occupied_seats_around(ferry, i) == 0 {
                    Cell::Occupied
                } else {
                    Cell::Seat
                }
            }
        })
        .collect();

    Ferry {
        cells,
        width: ferry.width,
        height: ferry.height,
    }
}

fn count_occupied_seats_around(ferry: &Ferry, cell_index: usize) -> u8 {
    let first_in_row = cell_index % ferry.width == 0;
    let last_in_row = cell_index % ferry.width == ferry.width - 1;

    let mut count: u8 = 0;
    if cell_index >= ferry.width {
        if ferry.cells[cell_index - ferry.width] == Cell::Occupied {
            count += 1;
        }
        if !first_in_row && ferry.cells[cell_index - ferry.width - 1] == Cell::Occupied {
            count += 1;
        }
        if !last_in_row && ferry.cells[cell_index - ferry.width + 1] == Cell::Occupied {
            count += 1;
        }
    }
    if !first_in_row && ferry.cells[cell_index - 1] == Cell::Occupied {
        count += 1;
    }
    if !last_in_row && ferry.cells[cell_index + 1] == Cell::Occupied {
        count += 1;
    }
    if cell_index / ferry.width < ferry.height - 1 {
        if ferry.cells[cell_index + ferry.width] == Cell::Occupied {
            count += 1;
        }
        if !first_in_row && ferry.cells[cell_index + ferry.width - 1] == Cell::Occupied {
            count += 1;
        }
        if !last_in_row && ferry.cells[cell_index + ferry.width + 1] == Cell::Occupied {
            count += 1;
        }
    }
    count
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
enum Cell {
    Floor,
    Seat,
    Occupied,
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
struct Ferry {
    cells: Vec<Cell>,
    height: usize,
    width: usize,
}

fn parse_input(input: &str) -> Result<Ferry, String> {
    if input.is_empty() {
        return Err("Unable to parse ferry: input is empty".to_owned());
    }
    let lines: Vec<&str> = input.split_terminator('\n').collect();
    let height = lines.len();
    let width = lines[0].len();
    if lines.iter().any(|l| l.len() != width) {
        return Err("Unable to parse ferry: not all rows have the same length".to_owned());
    }
    let cells = input
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| match c {
            '.' => Ok(Cell::Floor),
            'L' => Ok(Cell::Seat),
            '#' => Ok(Cell::Occupied),
            _ => Err(format!("Unable to parse cell '{}'", c)),
        })
        .collect::<Result<Vec<Cell>, String>>()?;

    Ok(Ferry {
        cells,
        height,
        width,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn next_gen_works_for_example() {
        // given
        let initial = parse_input(
            r"#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##",
        )
        .expect("Expected initial state to be parseable");
        let expected = parse_input(
            r"#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##",
        )
        .expect("Expected next generation state to be parseable");

        // when
        let result = next_gen(&initial);

        // then
        assert_eq!(result, expected);
    }

    #[test]
    fn seats_at_equilibrium_works_for_example() {
        // given
        let initial = parse_input(
            r"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL",
        )
        .expect("Expected initial state to be parseable");

        // when
        let result = seats_at_equilibrium(initial);

        // then
        assert_eq!(result, 37);
    }
}
