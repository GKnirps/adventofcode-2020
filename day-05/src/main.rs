use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(&Path::new(&filename)).map_err(|e| e.to_string())?;
    let lines: Vec<&str> = content.split('\n').filter(|l| !l.is_empty()).collect();

    let seat_ids = lines
        .iter()
        .enumerate()
        .map(|(i, l)| line_to_number(l, i))
        .collect::<Result<Vec<u16>, String>>()?;
    let max_seat_id = seat_ids
        .iter()
        .max()
        .ok_or_else(|| "No seat IDs in input".to_owned())?;

    println!("The highest seat ID is {}", max_seat_id);

    let own_seat = find_free_seat(&seat_ids).ok_or_else(|| "There is no free seat!".to_owned())?;
    println!("Your seat is {}", own_seat);

    Ok(())
}

fn find_free_seat(seat_ids: &[u16]) -> Option<u16> {
    let max_seat_id = *seat_ids.iter().max()?;

    let mut taken_seats: Vec<bool> = vec![false; max_seat_id as usize + 1];

    for seat in seat_ids {
        taken_seats[*seat as usize] = true;
    }

    taken_seats
        .windows(3)
        .enumerate()
        .filter(|(_, w)| w[0] && !w[1] && w[2])
        .map(|(i, _)| (i + 1) as u16)
        .next()
}

fn line_to_number(line: &str, line_index: usize) -> Result<u16, String> {
    if line.len() != 10 {
        return Err(format!("Line #{} is not 10 bytes long", line_index));
    }
    let mut n: u16 = 0;
    for c in line.chars() {
        n <<= 1;
        let bit = match c {
            'F' => 0,
            'B' => 1,
            'R' => 1,
            'L' => 0,
            _ => return Err(format!("Unknown character '{}' in line #{}", c, line_index)),
        };
        n += bit;
    }
    Ok(n)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn line_to_number_should_parse_correctly() {
        // given
        let line = "FBFBBFFRLR";

        // when
        let result = line_to_number(line, 42);

        // then
        assert_eq!(result, Ok(357));
    }

    #[test]
    fn line_to_number_should_fail_for_invalid_length() {
        // given
        let line = "FBFBFBFBFBF";

        // when
        let result = line_to_number(line, 42);

        // then
        assert_eq!(result, Err("Line #42 is not 10 bytes long".to_owned()));
    }

    #[test]
    fn line_to_number_should_fail_for_unknown_characters() {
        // given
        let line = "FBFBÖFBFR";

        // when
        let result = line_to_number(line, 42);

        // then
        assert_eq!(result, Err("Unknown character 'Ö' in line #42".to_owned()));
    }
}
