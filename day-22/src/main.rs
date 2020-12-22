#![feature(deque_range)]
use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(&Path::new(&filename)).map_err(|e| e.to_string())?;
    let (deck1, deck2) = parse_input(&content)?;

    let score = find_winner_score(deck1.clone(), deck2.clone());
    println!("Winner score is {}", score);

    let mut solved = HashMap::with_capacity(1000);
    let rc_deck = recursive_combat(deck1, deck2, &mut solved).1;
    println!("Recursive combat winner score is {}", score_deck(&rc_deck));

    Ok(())
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
enum Player {
    You,
    Crab,
}

fn recursive_combat(
    mut deck1: VecDeque<u64>,
    mut deck2: VecDeque<u64>,
    solved: &mut HashMap<(VecDeque<u64>, VecDeque<u64>), Player>,
) -> (Player, VecDeque<u64>) {
    let mut seen: HashSet<(VecDeque<u64>, VecDeque<u64>)> = HashSet::with_capacity(200);
    // ok, so how can I look up a tuple in a hashset without moving the values to the tuple?
    while !seen.contains(&(deck1.clone(), deck2.clone())) {
        if deck1.is_empty() {
            return (Player::Crab, deck2);
        } else if deck2.is_empty() {
            return (Player::You, deck1);
        }
        seen.insert((deck1.clone(), deck2.clone()));
        let card1 = deck1.pop_back().unwrap();
        let card2 = deck2.pop_back().unwrap();
        if (card1 as usize) > deck1.len() || (card2 as usize) > deck2.len() {
            if card1 > card2 {
                deck1.push_front(card1);
                deck1.push_front(card2);
            } else {
                deck2.push_front(card2);
                deck2.push_front(card1);
            }
        } else {
            let rec_deck1: VecDeque<u64> = deck1
                .range((deck1.len() - (card1 as usize))..)
                .copied()
                .collect();
            let rec_deck2: VecDeque<u64> = deck2
                .range((deck2.len() - (card2 as usize))..)
                .copied()
                .collect();
            // ok, so how can I look up a tuple in a hashset without moving the values to the tuple?
            let winner = solved
                .get(&(rec_deck1.clone(), rec_deck2.clone()))
                .copied()
                .unwrap_or_else(|| {
                    let (recursive_winner, _) =
                        recursive_combat(rec_deck1.clone(), rec_deck2.clone(), solved);
                    solved.insert((rec_deck1.clone(), rec_deck2.clone()), recursive_winner);
                    recursive_winner
                });
            match winner {
                Player::You => {
                    deck1.push_front(card1);
                    deck1.push_front(card2);
                }
                Player::Crab => {
                    deck2.push_front(card2);
                    deck2.push_front(card1);
                }
            }
        };
    }
    (Player::You, deck1)
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

    score_deck(&winner)
}

fn score_deck(deck: &VecDeque<u64>) -> u64 {
    deck.iter().zip(1..).map(|(card, pos)| card * pos).sum()
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

    #[test]
    fn recursive_combat_works_for_example() {
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
        let mut solved = HashMap::with_capacity(1000);
        let winner = recursive_combat(deck1, deck2, &mut solved);

        // then
        let winning_deck = winner.1;
        assert_eq!(score_deck(&winning_deck), 291);
    }
}
