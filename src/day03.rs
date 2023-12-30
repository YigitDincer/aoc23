use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Number {
    raw: usize,
    pos: Vec<(i32, i32)>,
}

type EngineSchema = HashMap<(i32, i32), char>;

fn parse_input(input: &str) -> EngineSchema {
    input
        .lines()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            (0..row.len())
                .filter(|&col_idx| row.as_bytes()[col_idx] as char != '.')
                .map(move |col_idx| {
                    (
                        (row_idx as i32, col_idx as i32),
                        row.as_bytes()[col_idx] as char,
                    )
                })
        })
        .collect()
}

fn get_all_non_numbers(input_as_hashmap: &EngineSchema) -> EngineSchema {
    input_as_hashmap
        .iter()
        .filter(|(_, ch)| !ch.is_digit(10))
        .map(|(&key, &ch)| (key, ch))
        .collect()
}

fn get_all_numbers(input_as_hashmap: &EngineSchema) -> EngineSchema {
    input_as_hashmap
        .iter()
        .filter(|(_, ch)| ch.is_digit(10))
        .map(|(&key, &ch)| (key, ch))
        .collect()
}

fn get_all_neighbor_pos(symbols: &EngineSchema) -> HashSet<(i32, i32)> {
    let mut neighbors = HashSet::new();

    symbols.iter().for_each(|((row, col), _)| {
        neighbors.insert((*row - 1, *col - 1));
        neighbors.insert((*row - 1, *col));
        neighbors.insert((*row - 1, *col + 1));

        neighbors.insert((*row, *col - 1));
        neighbors.insert((*row, *col + 1));

        neighbors.insert((*row + 1, *col - 1));
        neighbors.insert((*row + 1, *col));
        neighbors.insert((*row + 1, *col + 1));
    });

    neighbors
}

fn get_numbers(input: &str) -> HashSet<Number> {
    let re = Regex::new(r"\d+").unwrap();

    input
        .lines()
        .enumerate()
        .flat_map(|(row_idx, line)| {
            re.find_iter(line).map(move |mtch| {
                let pos = mtch.start();

                Number {
                    raw: mtch.as_str().parse().unwrap(),
                    pos: (pos..pos + mtch.len())
                        .map(|col_idx| (row_idx as i32, col_idx as i32))
                        .collect(),
                }
            })
        })
        .collect()
}

fn solve_1(input: &str) -> usize {
    let engine_shema = parse_input(input);
    let all_non_numbers = get_all_non_numbers(&engine_shema);
    let neighbors_of_non_numbers = get_all_neighbor_pos(&all_non_numbers);
    //let all_numbers = get_all_numbers(&engine_shema);
    let numbers = get_numbers(&input);

    numbers
        .iter()
        .filter(|number| {
            !HashSet::from_iter(number.pos.iter().cloned())
                .intersection(&neighbors_of_non_numbers)
                .collect::<Vec<_>>()
                .is_empty()
        })
        .map(|number| number.raw)
        .sum()
}

pub fn solve(input: &str) {
    println!("{}", solve_1(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "467
#..
35.";

    fn get_example_hashmap() -> EngineSchema {
        HashMap::from([
            ((2, 0), '3'),
            ((0, 2), '7'),
            ((0, 1), '6'),
            ((2, 1), '5'),
            ((0, 0), '4'),
            ((1, 0), '#'),
        ])
    }

    #[test]
    fn parse_input() {
        assert_eq!(super::parse_input(&EXAMPLE_INPUT), get_example_hashmap());
    }

    #[test]
    fn get_all_non_numbers() {
        let mut example_hashmap = get_example_hashmap();
        example_hashmap.retain(|_, &mut v| v != '#');

        assert_eq!(
            super::get_all_numbers(&get_example_hashmap()),
            example_hashmap
        );
    }

    #[test]
    fn solve() {
        assert_eq!(super::solve_1(&EXAMPLE_INPUT), 502);
    }

    #[test]
    fn get_numbers() {
        assert_eq!(
            super::get_numbers(&EXAMPLE_INPUT),
            HashSet::from([
                Number {
                    raw: 467,
                    pos: [(0, 0), (0, 1), (0, 2)].into()
                },
                Number {
                    raw: 35,
                    pos: [(2, 0), (2, 1)].into()
                }
            ])
        );
    }
}
