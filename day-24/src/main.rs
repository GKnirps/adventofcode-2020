use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(&Path::new(&filename)).map_err(|e| e.to_string())?;
    let tile_directions = parse_tiles(&content)?;

    let tile_positions = get_tile_positions(&tile_directions);
    let black_tiles = count_black_tiles(&tile_positions);
    println!("There are {} black tiles", black_tiles);

    Ok(())
}

fn count_black_tiles(tiles: &HashMap<(i64, i64), usize>) -> usize {
    tiles.values().filter(|n| **n % 2 == 1).count()
}

fn get_tile_positions(tile_dirs: &[Vec<Dir>]) -> HashMap<(i64, i64), usize> {
    tile_dirs.iter().map(|dirs| tile_pos(dirs)).fold(
        HashMap::with_capacity(tile_dirs.len()),
        |mut m, dir| {
            *m.entry(dir).or_insert(0) += 1;
            m
        },
    )
}

fn tile_pos(dirs: &[Dir]) -> (i64, i64) {
    dirs.iter()
        .map(|dir| match dir {
            Dir::E => (1, 0),
            Dir::W => (-1, 0),
            Dir::NE => (1, -1),
            Dir::NW => (0, -1),
            Dir::SE => (0, 1),
            Dir::SW => (-1, 1),
        })
        .fold((0, 0), |(px, py), (dx, dy)| (dx + px, dy + py))
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
enum Dir {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

fn parse_tiles(input: &str) -> Result<Vec<Vec<Dir>>, String> {
    input
        .split_terminator('\n')
        .map(parse_tile_directions)
        .collect()
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
enum Prefix {
    S,
    N,
}

fn parse_tile_directions(line: &str) -> Result<Vec<Dir>, String> {
    let mut directions = Vec::with_capacity(line.len());
    let mut prev = None;
    for c in line.chars() {
        match c {
            's' => {
                prev = {
                    if prev.is_none() {
                        Some(Prefix::S)
                    } else {
                        return Err(format!("Two consecutive prefixes in line {}", line));
                    }
                }
            }
            'n' => {
                prev = {
                    if prev.is_none() {
                        Some(Prefix::N)
                    } else {
                        return Err(format!("Two consecutive prefixes in line {}", line));
                    }
                }
            }
            'e' => {
                let dir = match prev {
                    Some(Prefix::N) => Dir::NE,
                    Some(Prefix::S) => Dir::SE,
                    None => Dir::E,
                };
                directions.push(dir);
                prev = None;
            }
            'w' => {
                let dir = match prev {
                    Some(Prefix::N) => Dir::NW,
                    Some(Prefix::S) => Dir::SW,
                    None => Dir::W,
                };
                directions.push(dir);
                prev = None;
            }
            _ => return Err(format!("Unknown direction: '{}'", c)),
        }
    }
    if prev.is_some() {
        Err(format!("Trailing prefix in line {}", line))
    } else {
        Ok(directions)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_tile_positions_works_for_example() {
        // given
        let input = r"sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";
        let tile_dirs = parse_tiles(input).expect("Expected example input to parse");

        // when
        let tile_positions = get_tile_positions(&tile_dirs);
        let black_tiles = count_black_tiles(&tile_positions);

        // then
        assert_eq!(black_tiles, 10);
    }

    #[test]
    fn tile_pos_works_for_examples() {
        // given
        let directions = parse_tile_directions("nwwswee").expect("expected example input to parse");

        // when
        let (px, py) = tile_pos(&directions);

        // then
        assert_eq!(px, 0);
        assert_eq!(py, 0);
    }
}
