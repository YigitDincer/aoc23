use itertools::Itertools;

#[derive(Debug, PartialEq, Clone)]
struct Hand {
    cards: Vec<u32>,
    bid: usize,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_hand_type = get_hand_type_with_jokers(&self.cards);
        let other_hand_type = get_hand_type_with_jokers(&other.cards);

        match self_hand_type.cmp(&other_hand_type) {
            std::cmp::Ordering::Equal => Some(self.cards.cmp(&other.cards)),
            other => Some(other),
        }
    }
}

fn convert_to_num_for_part1(character: char) -> u32 {
    match character {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => character.to_digit(10).unwrap(),
    }
}

fn convert_to_num_for_part2(character: char) -> u32 {
    match character {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        _ => character.to_digit(10).unwrap(),
    }
}

fn parse_hands(input: &str, converter: fn(char) -> u32) -> Vec<Hand> {
    input
        .lines()
        .map(|hand| hand.split_once(" ").unwrap())
        .map(|(cards_str, bid_str)| Hand {
            cards: cards_str.chars().map(|a| converter(a)).collect(),
            bid: bid_str.parse().unwrap(),
        })
        .collect()
}

fn get_ranked_hands(hands: &Vec<Hand>) -> Vec<Hand> {
    let mut hand_clone = hands.clone();
    hand_clone.sort_by(|a, b| a.partial_cmp(b).unwrap());
    hand_clone
}

fn get_hand_type_with_jokers(cards: &Vec<u32>) -> u32 {
    let a: u32 = 1;
    let joker_count = cards.iter().filter(|&card| card == &a).count();

    if joker_count == 0 {
        return get_hand_type(cards);
    }

    match get_hand_type(cards) {
        7 => return 7,
        6 => return 7, // at least one joker
        5 => return 7, // at least two jokers
        4 => return 6,
        3 => {
            if joker_count == 1 {
                return 5;
            } else {
                return 6;
            }
        }
        2 => return 4,
        1 => return 2,
        _ => panic!("You shouldn't be here"),
    }
}

fn get_hand_type(cards: &Vec<u32>) -> u32 {
    let mut cards_clone = cards.clone();
    cards_clone.sort();

    let unique_cards: Vec<_> = cards_clone.iter().unique().collect();
    let unique_count = unique_cards.len();

    match unique_count {
        5 => return 1,
        4 => return 2,
        3 => {
            // three of a kind
            if cards_clone.windows(3).any(|w| w[0] == w[1] && w[1] == w[2]) {
                return 4;
            }
            // two pairs
            return 3;
        }
        2 => {
            // four of a kind
            if cards_clone
                .windows(4)
                .any(|w| w[0] == w[1] && w[1] == w[2] && w[2] == w[3])
            {
                return 6;
            }

            // full house
            return 5;
        }
        1 => return 7,
        _ => panic!("You shouldn't be here!"),
    }
}

fn solve_1(input: &str) -> usize {
    get_ranked_hands(&parse_hands(input, convert_to_num_for_part1))
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) * hand.bid)
        .sum()
}

fn solve_2(input: &str) -> usize {
    get_ranked_hands(&parse_hands(input, convert_to_num_for_part2))
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) * hand.bid)
        .sum()
}

pub fn solve(input: &str) {
    //println!("{}", solve_1(input));
    println!("{}", solve_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    fn get_example_hands() -> Vec<Hand> {
        vec![
            Hand {
                cards: vec![3, 2, 10, 3, 13],
                bid: 765,
            },
            Hand {
                cards: vec![10, 5, 5, 11, 5],
                bid: 684,
            },
            Hand {
                cards: vec![13, 13, 6, 7, 7],
                bid: 28,
            },
            Hand {
                cards: vec![13, 10, 11, 11, 10],
                bid: 220,
            },
            Hand {
                cards: vec![12, 12, 12, 11, 14],
                bid: 483,
            },
        ]
    }

    #[test]
    fn parse_hands() {
        assert_eq!(
            super::parse_hands(&EXAMPLE_INPUT, convert_to_num_for_part1),
            get_example_hands()
        );
    }

    #[test]
    fn get_hand_types() {
        assert_eq!(
            get_example_hands()
                .iter()
                .map(|hand| get_hand_type(&hand.cards))
                .collect::<Vec<_>>(),
            vec![2, 4, 3, 3, 4]
        );
    }

    #[test]
    fn get_hand_type_after_jokers() {
        assert_eq!(
            get_example_hands()
                .iter()
                .map(|hand| {
                    let mut hand_cards = hand.cards.clone();
                    hand_cards.iter_mut().for_each(|num| {
                        if *num == 11 {
                            *num = 1;
                        }
                    });
                    super::get_hand_type_with_jokers(&hand_cards)
                })
                .collect::<Vec<_>>(),
            vec![2, 6, 3, 6, 6]
        );
    }

    #[test]
    fn test_hand_type_with_joker() {
        assert_eq!(get_hand_type_with_jokers(&vec![1, 1, 1, 1, 1]), 7);
        assert_eq!(get_hand_type_with_jokers(&vec![1, 1, 1, 1, 10]), 7);
        assert_eq!(get_hand_type_with_jokers(&vec![1, 1, 1, 10, 10]), 7);
        assert_eq!(get_hand_type_with_jokers(&vec![1, 1, 10, 10, 10]), 7);
        assert_eq!(get_hand_type_with_jokers(&vec![1, 10, 10, 10, 10]), 7);
        assert_eq!(get_hand_type_with_jokers(&vec![1, 2, 3, 4, 5]), 2);
        assert_eq!(get_hand_type_with_jokers(&vec![13, 12, 10, 10, 10]), 4);
    }

    #[test]
    fn solve_2() {
        assert_eq!(super::solve_2(&EXAMPLE_INPUT), 5905);
    }
}
