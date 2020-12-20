use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::read_to_string;
use std::path::Path;

const TILE_SIZE: usize = 10;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(&Path::new(&filename)).map_err(|e| e.to_string())?;
    let tiles = parse_tiles(&content)?;

    let tile_variants = tiles_rot_flipped(tiles);
    let img = solve_tile_puzzle(&tile_variants)?;
    let corner_product = prod_corner_ids(&img);
    println!(
        "Product of the corners of the solved puzzle: {}",
        corner_product
    );

    Ok(())
}

fn prod_corner_ids(img: &Img) -> u64 {
    if img.width == 0 {
        return 1;
    }
    [
        0,
        img.width - 1,
        img.width * (img.width - 1),
        img.width * img.width - 1,
    ]
    .iter()
    .map(|i| img.tiles[*i].id)
    .product()
}

// so I missed the part where the tiles can be rotated and flipped
// given the code I already wrote, it's easier to just add the rotated/flipped version of the tiles
// to the options (with the same tile ID, so only one of the variants is used)
fn tiles_rot_flipped(tiles: Vec<Tile>) -> Vec<Tile> {
    tiles
        .into_iter()
        // I can't get around this to_vec here for some reason
        .flat_map(|tile| tile_rot_flipped(tile).to_vec())
        .collect()
}

fn tile_rot_flipped(tile: Tile) -> [Tile; 8] {
    let rot90 = rotate_tile_counterclock(&tile);
    let rot180 = rotate_tile_counterclock(&rot90);
    let rot270 = rotate_tile_counterclock(&rot180);

    let flipped = flip_tile_hor(&tile);
    let flipped_rot90 = rotate_tile_counterclock(&flipped);
    let flipped_rot180 = rotate_tile_counterclock(&flipped_rot90);
    let flipped_rot270 = rotate_tile_counterclock(&flipped_rot180);

    [
        tile,
        rot90,
        rot180,
        rot270,
        flipped,
        flipped_rot90,
        flipped_rot180,
        flipped_rot270,
    ]
}

fn rotate_tile_counterclock(tile: &Tile) -> Tile {
    Tile {
        id: tile.id,
        border_top: tile.border_right,
        border_left: flip_border(tile.border_top),
        border_bottom: tile.border_left,
        border_right: flip_border(tile.border_bottom),
    }
}

fn flip_tile_hor(tile: &Tile) -> Tile {
    Tile {
        id: tile.id,
        border_top: flip_border(tile.border_top),
        border_left: tile.border_right,
        border_bottom: flip_border(tile.border_bottom),
        border_right: tile.border_left,
    }
}

fn flip_border(mut border: u16) -> u16 {
    let mut result = 0;
    for _ in 0..TILE_SIZE {
        result <<= 1;
        result |= border & 1;
        border >>= 1;
    }
    result
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
struct Img<'a> {
    tiles: Vec<&'a Tile>,
    width: usize,
}

fn solve_tile_puzzle(tiles: &[Tile]) -> Result<Img, String> {
    let img_width = stupid_int_sqrt(tiles.len() / 8)
        .ok_or_else(|| format!("Cannot order {} tiles into a square", tiles.len() / 8))?;
    let tiles_by_top: HashMap<u16, Vec<&Tile>> =
        tiles
            .iter()
            .fold(HashMap::with_capacity(tiles.len()), |mut map, tile| {
                map.entry(tile.border_top)
                    .or_insert_with(|| Vec::with_capacity(tiles.len()))
                    .push(tile);
                map
            });
    let tiles_by_left: HashMap<u16, Vec<&Tile>> =
        tiles
            .iter()
            .fold(HashMap::with_capacity(tiles.len()), |mut map, tile| {
                map.entry(tile.border_left)
                    .or_insert_with(|| Vec::with_capacity(tiles.len()))
                    .push(tile);
                map
            });
    let tiles_by_top_left: HashMap<(u16, u16), Vec<&Tile>> =
        tiles
            .iter()
            .fold(HashMap::with_capacity(tiles.len()), |mut map, tile| {
                map.entry((tile.border_top, tile.border_left))
                    .or_insert_with(|| Vec::with_capacity(tiles.len()))
                    .push(tile);
                map
            });
    let mut seen_tile_ids: HashSet<u64> = HashSet::with_capacity(tiles.len() / 8);
    let mut ordered_tiles: Vec<&Tile> = Vec::with_capacity(tiles.len() / 8);

    for tile in tiles {
        seen_tile_ids.insert(tile.id);
        ordered_tiles.push(tile);
        if append_matching_tiles(
            &mut ordered_tiles,
            &mut seen_tile_ids,
            img_width,
            &tiles_by_top,
            &tiles_by_left,
            &tiles_by_top_left,
        ) {
            return Ok(Img {
                width: img_width,
                tiles: ordered_tiles,
            });
        }
        ordered_tiles.pop();
        seen_tile_ids.remove(&tile.id);
    }

    Err("Unable to find a valid tile pattern".to_owned())
}

fn append_matching_tiles<'a>(
    ordered_tiles: &mut Vec<&'a Tile>,
    seen_tile_ids: &mut HashSet<u64>,
    img_width: usize,
    tiles_by_top: &HashMap<u16, Vec<&'a Tile>>,
    tiles_by_left: &HashMap<u16, Vec<&'a Tile>>,
    tiles_by_top_left: &HashMap<(u16, u16), Vec<&'a Tile>>,
) -> bool {
    // sanity check: method should not be called without an initial tile
    if ordered_tiles.is_empty() {
        return false;
    }
    if ordered_tiles.len() >= img_width * img_width {
        return true;
    }
    if ordered_tiles.len() % img_width == 0 {
        if let Some(tiles) =
            tiles_by_top.get(&ordered_tiles[ordered_tiles.len() - img_width].border_bottom)
        {
            for tile in tiles {
                if seen_tile_ids.contains(&tile.id) {
                    continue;
                }
                ordered_tiles.push(tile);
                seen_tile_ids.insert(tile.id);
                if append_matching_tiles(
                    ordered_tiles,
                    seen_tile_ids,
                    img_width,
                    tiles_by_top,
                    tiles_by_left,
                    tiles_by_top_left,
                ) {
                    return true;
                }
                ordered_tiles.pop();
                seen_tile_ids.remove(&tile.id);
            }
        }
    } else if ordered_tiles.len() / img_width == 0 {
        if let Some(tiles) = tiles_by_left.get(&ordered_tiles[ordered_tiles.len() - 1].border_right)
        {
            for tile in tiles {
                if seen_tile_ids.contains(&tile.id) {
                    continue;
                }
                ordered_tiles.push(tile);
                seen_tile_ids.insert(tile.id);
                if append_matching_tiles(
                    ordered_tiles,
                    seen_tile_ids,
                    img_width,
                    tiles_by_top,
                    tiles_by_left,
                    tiles_by_top_left,
                ) {
                    return true;
                }
                ordered_tiles.pop();
                seen_tile_ids.remove(&tile.id);
            }
        }
    } else if let Some(tiles) = tiles_by_top_left.get(&(
        ordered_tiles[ordered_tiles.len() - img_width].border_bottom,
        ordered_tiles[ordered_tiles.len() - 1].border_right,
    )) {
        for tile in tiles {
            if seen_tile_ids.contains(&tile.id) {
                continue;
            }
            ordered_tiles.push(tile);
            seen_tile_ids.insert(tile.id);
            if append_matching_tiles(
                ordered_tiles,
                seen_tile_ids,
                img_width,
                tiles_by_top,
                tiles_by_left,
                tiles_by_top_left,
            ) {
                return true;
            }
            ordered_tiles.pop();
            seen_tile_ids.remove(&tile.id);
        }
    }
    false
}

fn stupid_int_sqrt(value: usize) -> Option<usize> {
    for root in 1..(value / 2) {
        if root * root == value {
            return Some(root);
        }
    }
    None
}

fn parse_tiles(input: &str) -> Result<Vec<Tile>, String> {
    input.split_terminator("\n\n").map(parse_tile).collect()
}

fn parse_tile(input: &str) -> Result<Tile, String> {
    let mut lines = input.split_terminator('\n');
    let id = lines
        .next()
        .expect("Expected tile ID line")
        .trim_start_matches("Tile ")
        .trim_end_matches(':')
        .parse::<u64>()
        .map_err(|e| format!("Unable to parse tile ID: {}", e))?;
    let pixel: Vec<bool> = lines
        .map(|line| line.chars())
        .flatten()
        .map(|c| c == '#')
        .collect();
    if pixel.len() != TILE_SIZE * TILE_SIZE {
        return Err(format!(
            "Expected tile to contain {} pixel but was {}",
            TILE_SIZE * TILE_SIZE,
            pixel.len()
        ));
    }

    let mut border_top = 0;
    let mut border_left = 0;
    let mut border_bottom = 0;
    let mut border_right = 0;

    for i in 0..TILE_SIZE {
        border_top <<= 1;
        border_left <<= 1;
        border_bottom <<= 1;
        border_right <<= 1;
        if pixel[i] {
            border_top |= 1;
        }
        if pixel[i + TILE_SIZE * (TILE_SIZE - 1)] {
            border_bottom |= 1;
        }
        if pixel[TILE_SIZE * i] {
            border_left |= 1;
        }
        if pixel[TILE_SIZE - 1 + TILE_SIZE * i] {
            border_right |= 1;
        }
    }

    Ok(Tile {
        id,
        border_top,
        border_left,
        border_bottom,
        border_right,
    })
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
struct Tile {
    id: u64,
    border_top: u16,
    border_left: u16,
    border_bottom: u16,
    border_right: u16,
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &str = r"Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...

";

    #[test]
    fn rotate_tile_counterclock_rotates_correctly() {
        // given
        let tile = Tile {
            id: 42,
            border_top: 0b0001,
            border_left: 0b0011,
            border_bottom: 0b0111,
            border_right: 0b1111,
        };

        // when
        let rotated = rotate_tile_counterclock(&tile);

        assert_eq!(
            rotated,
            Tile {
                id: 42,
                border_top: 0b0000001111,
                border_left: 0b1000000000,
                border_bottom: 0b0000000011,
                border_right: 0b1110000000,
            }
        )
    }

    #[test]
    fn flip_tile_hor_flips_correctly() {
        // given
        let tile = Tile {
            id: 42,
            border_top: 0b0001,
            border_left: 0b0011,
            border_bottom: 0b0111,
            border_right: 0b1111,
        };

        // when
        let flipped = flip_tile_hor(&tile);

        // then
        assert_eq!(
            flipped,
            Tile {
                id: 42,
                border_top: 0b1000000000,
                border_left: 0b1111,
                border_bottom: 0b1110000000,
                border_right: 0b0011,
            }
        )
    }

    #[test]
    fn solve_tile_puzzle_works_for_example() {
        // given
        let tiles = parse_tiles(EXAMPLE_INPUT).expect("expected example input to parse");
        let tile_variants = tiles_rot_flipped(tiles);

        // when
        let result = solve_tile_puzzle(&tile_variants).expect("expected puzzle to be solved");

        // then
        assert_eq!(result.width, 3);
        assert_eq!(result.tiles.len(), 9);
        assert_eq!(prod_corner_ids(&result), 20899048083289);
    }
}
