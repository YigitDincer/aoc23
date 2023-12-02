const CUBES_IN_BAG: Cubes = Cubes {
    red: 12,
    green: 13,
    blue: 14,
};

fn parse_game_line(game_line: &str) -> Game {
    let (game_info, contents_str) = game_line.split_once(": ").unwrap();
    let (_, game_id) = game_info.split_once(' ').unwrap();

    let contents: Vec<_> = contents_str
        .split("; ")
        .map(|content_str| {
            let mut content = Cubes::default();
            content_str.split(", ").for_each(|one_pair| {
                let (num, color) = one_pair.split_once(' ').unwrap();
                let num = num.parse().unwrap();
                match color {
                    "red" => content.red = num,
                    "green" => content.green = num,
                    "blue" => content.blue = num,
                    _ => panic!("wrong color"),
                }
            });
            content
        })
        .collect();

    Game {
        id: game_id.parse().unwrap(),
        draws: contents,
    }
}

fn get_min_cubes_required(draws: &[Cubes]) -> Cubes {
    let mut min_cubes = Cubes::default();

    draws.iter().for_each(|draw| {
        if min_cubes.red < draw.red {
            min_cubes.red = draw.red
        }

        if min_cubes.green < draw.green {
            min_cubes.green = draw.green
        }

        if min_cubes.blue < draw.blue {
            min_cubes.blue = draw.blue
        }
    });

    min_cubes
}

fn is_draw_possible(draw: &Cubes, reference: &Cubes) -> bool {
    reference.red >= draw.red && reference.green >= draw.green && reference.blue >= draw.blue
}

fn solve_1(input: &str) -> u32 {
    input
        .lines()
        .map(parse_game_line)
        .map(|game| (get_min_cubes_required(&game.draws), game.id))
        .filter(|(cubes, _)| is_draw_possible(&cubes, &CUBES_IN_BAG))
        .map(|(_, id)| id)
        .sum()
}

fn solve_2(input: &str) -> u32 {
    input
        .lines()
        .map(parse_game_line)
        .map(|game| get_min_cubes_required(&game.draws))
        .map(|cubes| cubes.red * cubes.green * cubes.blue)
        .sum()
}

pub fn solve(input: &str) {
    println!("{}", solve_1(input));
    println!("{}", solve_2(input));
}

#[derive(PartialEq, Eq, Debug, Default)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(PartialEq, Debug)]
struct Game {
    id: u32,
    draws: Vec<Cubes>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const GAME_1_DRAWS: [Cubes; 3] = [
        Cubes {
            red: 4,
            green: 0,
            blue: 3,
        },
        Cubes {
            red: 1,
            green: 2,
            blue: 6,
        },
        Cubes {
            red: 0,
            green: 2,
            blue: 0,
        },
    ];

    const EXAMPLE_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn example() {
        assert_eq!(
            parse_game_line(EXAMPLE_INPUT.lines().next().unwrap()),
            Game {
                id: 1,
                draws: GAME_1_DRAWS.into(),
            }
        );
    }

    #[test]
    fn count_valid_game_ids() {
        assert_eq!(solve_1(EXAMPLE_INPUT), 8);
    }

    #[test]
    fn get_min_cubes_required() {
        assert_eq!(
            super::get_min_cubes_required(&GAME_1_DRAWS),
            Cubes {
                red: 4,
                green: 2,
                blue: 6
            }
        );
    }

    #[test]
    fn solve_2() {
        assert_eq!(super::solve_2(EXAMPLE_INPUT), 2286);
    }
}
