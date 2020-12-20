use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fmt;
use std::fs::read_to_string;
use std::path::Path;

const TILE_SIZE: usize = 10;
const TILE_SIZE_IMG: usize = TILE_SIZE - 2;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(&Path::new(&filename)).map_err(|e| e.to_string())?;
    let tiles = parse_tiles(&content)?;

    let tile_variants = tiles_rot_flipped(tiles);
    let tiled_img = solve_tile_puzzle(&tile_variants)?;
    let corner_product = prod_corner_ids(&tiled_img);
    println!(
        "Product of the corners of the solved puzzle: {}",
        corner_product
    );

    let img = img_from_tiled_img(&tiled_img);
    let roughness = count_water_roughness(&img);
    println!("The water roughness value is {}", roughness);

    Ok(())
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
struct Img {
    img_data: Vec<bool>,
    width: usize,
    height: usize,
}

impl fmt::Display for Img {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.img_data.chunks_exact(self.width) {
            for px in row {
                if *px {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// important assumption: sea serpents do not overlap
fn count_water_roughness(photo: &Img) -> usize {
    let patterns = create_all_sea_serpent_patterns();
    let n_sea_serpents = count_sea_serpents(photo, &patterns);
    let total_white_pixels = photo.img_data.iter().filter(|p| **p).count();
    let sea_searpent_white_pixels = patterns[0].img_data.iter().filter(|p| **p).count();

    total_white_pixels - n_sea_serpents * sea_searpent_white_pixels
}

// Important: counts overlapping sea serpents as well
fn count_sea_serpents(photo: &Img, patterns: &[Img]) -> usize {
    patterns
        .iter()
        .map(|pattern| {
            let mut count: usize = 0;
            for top in 0..=(photo.height - pattern.height) {
                for left in 0..=(photo.width - pattern.width) {
                    if pattern_matches_at_pos(photo, pattern, top, left) {
                        count += 1;
                    }
                }
            }
            count
        })
        .max()
        .unwrap_or(0)
}

fn pattern_matches_at_pos(img: &Img, pattern: &Img, top: usize, left: usize) -> bool {
    if img.height < top + pattern.height || img.width < left + pattern.width {
        return false;
    }
    for y in 0..pattern.height {
        for x in 0..pattern.width {
            if pattern.img_data[y * pattern.width + x]
                && !img.img_data[left + x + img.width * (top + y)]
            {
                return false;
            }
        }
    }
    true
}

fn create_all_sea_serpent_patterns() -> [Img; 8] {
    let pattern = create_sea_serpent_pattern();
    let rot90 = rot_img_counterclock(&pattern);
    let rot180 = rot_img_counterclock(&rot90);
    let rot270 = rot_img_counterclock(&rot180);

    let flipped = flip_img_hor(&pattern);
    let flip_rot90 = rot_img_counterclock(&flipped);
    let flip_rot180 = rot_img_counterclock(&flip_rot90);
    let flip_rot270 = rot_img_counterclock(&flip_rot180);

    [
        pattern,
        rot90,
        rot180,
        rot270,
        flipped,
        flip_rot90,
        flip_rot180,
        flip_rot270,
    ]
}

fn create_sea_serpent_pattern() -> Img {
    let raw = "                  # #    ##    ##    ### #  #  #  #  #  #   ";
    let img_data: Vec<bool> = raw.chars().map(|c| c == '#').collect();
    Img {
        img_data,
        width: 20,
        height: 3,
    }
}

fn rot_img_counterclock(img: &Img) -> Img {
    let img_data = rotate_img_data_counterclock(&img.img_data, img.width, img.height);
    Img {
        img_data,
        width: img.height,
        height: img.width,
    }
}

fn flip_img_hor(img: &Img) -> Img {
    let img_data = flip_img_data_hor(&img.img_data, img.width, img.height);
    Img {
        img_data,
        width: img.width,
        height: img.height,
    }
}

fn img_from_tiled_img(tiled_img: &TiledImg) -> Img {
    let width = tiled_img.width * TILE_SIZE_IMG;
    let height = width;
    let mut img_data = vec![false; width * height];
    for tile_y in 0..tiled_img.width {
        for tile_x in 0..tiled_img.width {
            for tile_px_y in 0..TILE_SIZE_IMG {
                for tile_px_x in 0..TILE_SIZE_IMG {
                    let y = tile_y * TILE_SIZE_IMG + tile_px_y;
                    let x = tile_x * TILE_SIZE_IMG + tile_px_x;
                    img_data[x + y * width] = tiled_img.tiles[tile_x + tile_y * tiled_img.width]
                        .img_data[tile_px_x + tile_px_y * TILE_SIZE_IMG];
                }
            }
        }
    }
    Img {
        img_data,
        width,
        height,
    }
}

fn prod_corner_ids(img: &TiledImg) -> u64 {
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
        img_data: rotate_img_data_counterclock(&tile.img_data, TILE_SIZE_IMG, TILE_SIZE_IMG),
    }
}

fn rotate_img_data_counterclock(data: &[bool], width: usize, height: usize) -> Vec<bool> {
    let mut rotated: Vec<bool> = vec![false; width * height];
    for y in 0..height {
        for x in 0..width {
            let rot_x = y;
            let rot_y = width - 1 - x;
            rotated[rot_x + rot_y * height] = data[x + y * width];
        }
    }
    rotated
}

fn flip_tile_hor(tile: &Tile) -> Tile {
    Tile {
        id: tile.id,
        border_top: flip_border(tile.border_top),
        border_left: tile.border_right,
        border_bottom: flip_border(tile.border_bottom),
        border_right: tile.border_left,
        img_data: flip_img_data_hor(&tile.img_data, TILE_SIZE_IMG, TILE_SIZE_IMG),
    }
}

fn flip_img_data_hor(data: &[bool], width: usize, height: usize) -> Vec<bool> {
    let mut flipped: Vec<bool> = vec![false; width * height];
    for y in 0..height {
        for x in 0..width {
            let flip_x = width - 1 - x;
            let flip_y = y;
            flipped[flip_x + flip_y * width] = data[x + y * width];
        }
    }
    flipped
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
struct TiledImg<'a> {
    tiles: Vec<&'a Tile>,
    width: usize,
}

fn solve_tile_puzzle(tiles: &[Tile]) -> Result<TiledImg, String> {
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
            return Ok(TiledImg {
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
    let img_data = crop_tile_image_data(&pixel);

    Ok(Tile {
        id,
        border_top,
        border_left,
        border_bottom,
        border_right,
        img_data,
    })
}

fn crop_tile_image_data(full_tile: &[bool]) -> Vec<bool> {
    (0..TILE_SIZE_IMG * TILE_SIZE_IMG)
        .map(|i| {
            let x = i % TILE_SIZE_IMG + 1;
            let y = i / TILE_SIZE_IMG + 1;
            full_tile[x + y * TILE_SIZE]
        })
        .collect()
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
struct Tile {
    id: u64,
    border_top: u16,
    border_left: u16,
    border_bottom: u16,
    border_right: u16,
    img_data: Vec<bool>,
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
        let mut img_data = vec![false; TILE_SIZE_IMG * TILE_SIZE_IMG];
        img_data[0] = true;
        img_data[TILE_SIZE_IMG + 1] = true;
        let tile = Tile {
            id: 42,
            border_top: 0b0001,
            border_left: 0b0011,
            border_bottom: 0b0111,
            border_right: 0b1111,
            img_data,
        };

        let mut expected_img_data = vec![false; TILE_SIZE_IMG * TILE_SIZE_IMG];
        expected_img_data[(TILE_SIZE_IMG - 1) * TILE_SIZE_IMG] = true;
        expected_img_data[1 + TILE_SIZE_IMG * (TILE_SIZE_IMG - 2)] = true;

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
                img_data: expected_img_data,
            }
        )
    }

    #[test]
    fn flip_tile_hor_flips_correctly() {
        // given
        let mut img_data = vec![false; TILE_SIZE_IMG * TILE_SIZE_IMG];
        img_data[0] = true;
        img_data[TILE_SIZE_IMG + 1] = true;
        let tile = Tile {
            id: 42,
            border_top: 0b0001,
            border_left: 0b0011,
            border_bottom: 0b0111,
            border_right: 0b1111,
            img_data,
        };

        let mut expected_img_data = vec![false; TILE_SIZE_IMG * TILE_SIZE_IMG];
        expected_img_data[(TILE_SIZE_IMG - 1)] = true;
        expected_img_data[TILE_SIZE_IMG + (TILE_SIZE_IMG - 2)] = true;

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
                img_data: expected_img_data,
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

    #[test]
    fn count_water_roughness_works_for_example() {
        // given
        let tiles = parse_tiles(EXAMPLE_INPUT).expect("expected example input to parse");
        let tile_variants = tiles_rot_flipped(tiles);
        let tiled_img = solve_tile_puzzle(&tile_variants).expect("expected puzzle to be solved");
        let img = img_from_tiled_img(&tiled_img);

        // when
        println!("{}", img);
        let roughness = count_water_roughness(&img);

        // then
        assert_eq!(roughness, 273);
    }
}
