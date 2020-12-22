use std::collections::VecDeque;
use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(&Path::new(&filename)).map_err(|e| e.to_string())?;
    let (deck1, deck2) = parse_input(&content)?;

    let score = find_winner_score(deck1, deck2);
    println!("Winner score is {}", score);

    Ok(())
}

fn find_winner_score(mut deck1: VecDeque<u64>, mut deck2: VecDeque<u64>) -> u64 {
    while !deck1.is_empty() && !deck2.is_empty() {
        let card1 = deck1.pop_back().unwrap();
        let card2 = deck2.pop_back().unwrap();
        if card1 > card2 {
            deck1.push_front(card1);
            deck1.push_front(card2);
        } else {
            deck2.push_front(card2);
            deck2.push_front(card1);
        }
    }

    let winner = if deck1.len() < deck2.len() {
        deck2
    } else {
        deck1
    };

    winner.iter().zip(1..).map(|(card, pos)| card * pos).sum()
}

fn parse_input(input: &str) -> Result<(VecDeque<u64>, VecDeque<u64>), String> {
    let mut split = input.split_terminator("\n\n");
    let deck1 = parse_deck(
        split
            .next()
            .ok_or_else(|| "Expected first deck".to_owned())?
            .trim_start_matches("Player 1:\n"),
    )?;
    let deck2 = parse_deck(
        split
            .next()
            .ok_or_else(|| "Expected first deck".to_owned())?
            .trim_start_matches("Player 2:\n"),
    )?;

    Ok((deck1, deck2))
}

fn parse_deck(input: &str) -> Result<VecDeque<u64>, String> {
    input
        .split_terminator('\n')
        .rev()
        .map(|s| {
            s.parse::<u64>()
                .map_err(|e| format!("Unable to parse card '{}': {}", s, e))
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn find_winner_score_works_for_example() {
        // given
        let (deck1, deck2) = parse_input(
            r"Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10",
        )
        .expect("Expected example input to parse");

        // when
        let score = find_winner_score(deck1, deck2);

        // then
        assert_eq!(score, 306);
    }
}
