use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(&Path::new(&filename)).map_err(|e| e.to_string())?;
    let (door_pubkey, card_pubkey) = parse_input(&content)?;

    let encryption_key = find_encryption_key(door_pubkey, card_pubkey);
    println!("Encryption key is: {}", encryption_key);

    Ok(())
}

fn parse_input(input: &str) -> Result<(u64, u64), String> {
    let mut iter = input.split_terminator('\n');
    let door_pubkey = iter
        .next()
        .ok_or_else(|| "Unable to find door public key".to_owned())?
        .parse::<u64>()
        .map_err(|e| format!("Invalid door public key: {}", e))?;
    let card_pubkey = iter
        .next()
        .ok_or_else(|| "Unable to find card public key".to_owned())?
        .parse::<u64>()
        .map_err(|e| format!("Invalid card public key: {}", e))?;
    Ok((door_pubkey, card_pubkey))
}

fn find_loop_size(subject_number: u64, transformed: u64) -> u64 {
    let mut value = 1;
    for i in 1.. {
        value *= subject_number;
        value %= 20201227;
        if value == transformed {
            return i;
        }
    }
    0
}

fn transform(subject_number: u64, loop_size: u64) -> u64 {
    let mut value = 1;
    for _ in 0..loop_size {
        value *= subject_number;
        value %= 20201227;
    }
    value
}

fn find_encryption_key(door_pubkey: u64, card_pubkey: u64) -> u64 {
    let door_loop = find_loop_size(7, door_pubkey);
    transform(card_pubkey, door_loop)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn find_loop_size_works_for_examples() {
        assert_eq!(find_loop_size(7, 5764801), 8);
        assert_eq!(find_loop_size(7, 17807724), 11);
    }

    #[test]
    fn find_encryption_key_works_for_example() {
        assert_eq!(find_encryption_key(5764801, 17807724), 14897079);
    }
}
