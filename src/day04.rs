use std::collections::HashSet;

#[derive(Debug, PartialEq)]
struct Card {
    winning_numbers: Vec<u32>,
    guess: Vec<u32>,
}

fn parse(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|line| {
            let (card_id_and_winning_nos, guess_str) = line.split_once("|").unwrap();
            let (_, winning_nos) = card_id_and_winning_nos.split_once(":").unwrap();

            Card {
                winning_numbers: winning_nos
                    .trim()
                    .split_ascii_whitespace()
                    .map(|number| number.parse().unwrap())
                    .collect(),
                guess: guess_str
                    .trim()
                    .split_ascii_whitespace()
                    .map(|number| number.parse().unwrap())
                    .collect(),
            }
        })
        .collect()
}

fn get_count_of_matches(card: &Card) -> usize {
    let winning_nos: HashSet<u32> = HashSet::from_iter(card.winning_numbers.iter().cloned());
    let guess = HashSet::from_iter(card.guess.iter().cloned());
    winning_nos.intersection(&guess).count()
}

fn calculate_points(card: &Card) -> u32 {
    let base: u32 = 2;
    base.pow(get_count_of_matches(card) as u32 - 1)
}

fn solve_1(input: &str) -> u32 {
    parse(input).iter().map(|card| calculate_points(card)).sum()
}

fn solve_2(input: &str) -> usize {
    count_of_total_cards(&parse(input)).iter().sum()
}

fn count_of_total_cards(cards: &[Card]) -> Vec<usize> {
    let mut count_of_cards = vec![1; cards.len()];

    let count_of_matches_per_card: Vec<_> = cards
        .iter()
        .map(|card| get_count_of_matches(card))
        .collect();

    for (idx, &count) in count_of_matches_per_card.iter().enumerate() {
        for i in 1..=count {
            count_of_cards[idx + i] += count_of_cards[idx];
        }
    }

    count_of_cards
}

pub fn solve(input: &str) {
    println!("{}", solve_1(input));
    println!("{}", solve_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    fn get_example_cards() -> Vec<Card> {
        vec![
            Card {
                winning_numbers: vec![41, 48, 83, 86, 17],
                guess: vec![83, 86, 6, 31, 17, 9, 48, 53],
            },
            Card {
                winning_numbers: vec![13, 32, 20, 16, 61],
                guess: vec![61, 30, 68, 82, 17, 32, 24, 19],
            },
            Card {
                winning_numbers: vec![1, 21, 53, 59, 44],
                guess: vec![69, 82, 63, 72, 16, 21, 14, 1],
            },
            Card {
                winning_numbers: vec![41, 92, 73, 84, 69],
                guess: vec![59, 84, 76, 51, 58, 5, 54, 83],
            },
            Card {
                winning_numbers: vec![87, 83, 26, 28, 32],
                guess: vec![88, 30, 70, 12, 93, 22, 82, 36],
            },
            Card {
                winning_numbers: vec![31, 18, 13, 56, 72],
                guess: vec![74, 77, 10, 23, 35, 67, 36, 11],
            },
        ]
    }

    #[test]
    fn example() {
        assert_eq!(parse(EX_INPUT), get_example_cards());
    }

    #[test]
    fn get_count_of_matches() {
        assert_eq!(super::get_count_of_matches(&get_example_cards()[0]), 4);
    }

    #[test]
    fn calculate_points() {
        assert_eq!(super::calculate_points(&get_example_cards()[0]), 8);
    }

    #[test]
    fn count_of_total_cards() {
        assert_eq!(
            super::count_of_total_cards(&get_example_cards()),
            [1, 2, 4, 8, 14, 1]
        );
    }
}
