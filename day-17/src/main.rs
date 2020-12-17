use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(&Path::new(&filename)).map_err(|e| e.to_string())?;
    let (input_plane, input_width) = parse_input(&content);

    // we already know the maximum number of generations, so we don't need to bother with resizing, we can
    // just add enough padding left and right to last for at least six generations without overflowing
    let initial_world_for_part_1 = create_world_with_centered_plane(&input_plane, input_width, 6);
    let world_after_6_gen = run_generations(initial_world_for_part_1, 6);
    let alive_count_after_6_gen = count_alive_cells(&world_after_6_gen);
    println!(
        "After six generations, {} cubes are active",
        alive_count_after_6_gen
    );

    Ok(())
}

fn count_alive_cells(world: &World) -> usize {
    world.cells.iter().filter(|c| **c).count()
}

fn run_generations(initial_world: World, generations: usize) -> World {
    let mut world = initial_world;
    for _ in 0..generations {
        world = next_gen(&world);
    }
    world
}

fn next_gen(world: &World) -> World {
    let next_cells: Vec<bool> = world
        .cells
        .iter()
        .enumerate()
        .map(|(i, alive)| {
            let alive_neighbours = count_alive_neighbours(world, i as isize);
            alive_neighbours == 3 || (alive_neighbours == 2 && *alive)
        })
        .collect();

    World {
        cells: next_cells,
        size_x: world.size_x,
        size_y: world.size_y,
        size_z: world.size_z,
    }
}

fn count_alive_neighbours(world: &World, cell_index: isize) -> u8 {
    let mut count = 0;

    for dz in -1..=1 {
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 && dz == 0 {
                    continue;
                }
                let index = cell_index
                    + dz * world.size_y as isize * world.size_x as isize
                    + dy * world.size_x as isize
                    + dx;
                if index > 0 && (index as usize) < world.cells.len() && world.cells[index as usize]
                {
                    count += 1;
                }
            }
        }
    }

    count
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
struct World {
    cells: Vec<bool>,
    size_x: usize,
    size_y: usize,
    size_z: usize,
}

fn create_world_with_centered_plane(
    plane_cells: &[bool],
    plane_width: usize,
    padding: usize,
) -> World {
    let size_x = plane_width + 2 * padding;
    let size_y = plane_cells.len() / plane_width + 2 * padding;
    let size_z = 1 + 2 * padding;

    let mut cells = vec![false; size_x * size_y * size_z];

    for (row_index, row) in plane_cells.chunks_exact(plane_width).enumerate() {
        for (col_index, cell) in row.iter().enumerate() {
            cells[size_x * size_y * padding
                + size_x * (row_index + padding)
                + col_index
                + padding] = *cell;
        }
    }

    World {
        cells,
        size_x,
        size_y,
        size_z,
    }
}

fn parse_input(input: &str) -> (Vec<bool>, usize) {
    // less error checking today, just assume the input is not malformed and all lines have the same length
    let width = input
        .split('\n')
        .next()
        .expect("Expected at least one line")
        .len();
    let cells: Vec<bool> = input
        .chars()
        .filter(|c| *c == '#' || *c == '.')
        .map(|c| c == '#')
        .collect();

    (cells, width)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run_generations_works_for_example() {
        // given
        let (init_plane, init_width) = parse_input(
            r".#.
..#
###",
        );
        let world = create_world_with_centered_plane(&init_plane, init_width, 6);

        // when
        let result_world = run_generations(world, 6);
        let alive_count = count_alive_cells(&result_world);

        // then
        assert_eq!(alive_count, 112);
    }
}
