#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Tile {
    Empty,
    Wall,
    Rock,
}

enum Direction {
    North,
    South,
    East,
    West,
}

type Platform = Vec<Vec<Tile>>;

fn get_total_load(platform: &Platform) -> usize {
    let platform_height = platform.len();

    platform
        .iter()
        .enumerate()
        .map(|(row_idx, row)| {
            (
                row_idx,
                row.iter().filter(|tile| tile == &&Tile::Rock).count(),
            )
        })
        .map(|(row_idx, count)| count * (platform_height - row_idx))
        .sum()
}

fn tilt(platform: &Platform, direction: Direction) -> Platform {
    let mut tilted_platform = platform.clone();

    let move_vector: (i32, i32) = match direction {
        Direction::North => (0, -1),
        Direction::South => (0, 1),
        Direction::East => (1, 0),
        Direction::West => (-1, 0),
    };

    let width = platform.first().unwrap().len();
    let height = platform.len();

    for y in 0..height {
        for x in 0..width {
            if tilted_platform[y][x] == Tile::Rock {
                let mut last_x = x;
                let mut last_y = y;

                for i in 1.. {
                    let new_x = x as i32 + i * move_vector.0;
                    let new_y = y as i32 + i * move_vector.1;

                    if new_x < 0 || new_x >= width as i32 || new_y < 0 || new_y >= height as i32 {
                        if last_x != x || last_y != y {
                            tilted_platform[last_y][last_x] = Tile::Rock;
                            tilted_platform[y][x] = Tile::Empty;
                        }
                        break;
                    }

                    if tilted_platform[new_y as usize][new_x as usize] == Tile::Empty {
                        last_x = new_x as usize;
                        last_y = new_y as usize;
                        continue;
                    } else {
                        if last_x != x || last_y != y {
                            tilted_platform[last_y][last_x] = Tile::Rock;
                            tilted_platform[y][x] = Tile::Empty;
                        }
                        break;
                    }
                }
            }
        }
    }

    tilted_platform
}

fn parse_into_platform(input: &str) -> Platform {
    let mut platform: Platform = Vec::new();

    for line in input.lines() {
        let mut row: Vec<Tile> = Vec::new();
        for c in line.chars() {
            let tile = match c {
                '.' => Tile::Empty,
                '#' => Tile::Wall,
                'O' => Tile::Rock,
                _ => panic!("Unknown tile type"),
            };
            row.push(tile);
        }
        platform.push(row);
    }

    platform
}

fn solve_1(input: &str) -> usize {
    get_total_load(&tilt(&parse_into_platform(&input), Direction::North))
}

fn tilt_one_cycle(platform: &Platform) -> Platform {
    let mut initial_tilt = platform.clone();
    initial_tilt = tilt(&initial_tilt, Direction::North);
    initial_tilt = tilt(&initial_tilt, Direction::West);
    initial_tilt = tilt(&initial_tilt, Direction::South);
    initial_tilt = tilt(&initial_tilt, Direction::East);
    initial_tilt
}

fn solve_2(input: &str) -> usize {
    let mut initial_tilt = parse_into_platform(&input);
    let mut last_platform: Platform = initial_tilt.clone();

    for _ in 1..1000000000 {
        initial_tilt = tilt_one_cycle(&initial_tilt);

        if last_platform != initial_tilt {
            last_platform = initial_tilt.clone();
        } else {
            break;
        }
    }

    get_total_load(&initial_tilt)
}

pub fn solve(input: &str) {
    println!("{}", solve_1(input));
    println!("{}", solve_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_INPUT: &str = "O..
#..
OO.";

    fn get_parsed_small_input() -> Platform {
        let mut platform = vec![vec![Tile::Empty; 3]; 3];

        platform[0][0] = Tile::Rock;
        platform[0][1] = Tile::Empty;
        platform[0][2] = Tile::Empty;
        platform[1][0] = Tile::Wall;
        platform[1][1] = Tile::Empty;
        platform[1][2] = Tile::Empty;
        platform[2][0] = Tile::Rock;
        platform[2][1] = Tile::Rock;
        platform[2][2] = Tile::Empty;

        platform
    }

    const SMALL_INPUT_2: &str = "O..
#O.
O..";

    fn get_parsed_small_input_2_after_tilt() -> Platform {
        let mut platform = vec![vec![Tile::Empty; 3]; 3];

        platform[0][0] = Tile::Rock;
        platform[0][1] = Tile::Rock;
        platform[0][2] = Tile::Empty;
        platform[1][0] = Tile::Wall;
        platform[1][1] = Tile::Empty;
        platform[1][2] = Tile::Empty;
        platform[2][0] = Tile::Rock;
        platform[2][1] = Tile::Empty;
        platform[2][2] = Tile::Empty;

        platform
    }

    const EXAMPLE_INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    const TILTED_EXAMPLE_INPUT: &str = "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";

    const EXAMPLE_INPUT_AFTER_1_CYCLE: &str = ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....";

    #[test]
    fn solve_1() {
        assert_eq!(super::solve_1(&EXAMPLE_INPUT), 136);
    }

    #[test]
    fn solve_2() {
        assert_eq!(super::solve_2(&EXAMPLE_INPUT), 64);
    }

    #[test]
    fn parse_into_platform() {
        assert_eq!(
            super::parse_into_platform(&SMALL_INPUT),
            get_parsed_small_input()
        );
    }

    #[test]
    fn tilt_small_example() {
        assert_eq!(
            super::tilt(
                &super::parse_into_platform(&SMALL_INPUT_2),
                Direction::North
            ),
            get_parsed_small_input_2_after_tilt()
        );
    }

    #[test]
    fn tilt() {
        assert_eq!(
            super::tilt(
                &super::parse_into_platform(&EXAMPLE_INPUT),
                Direction::North
            ),
            super::parse_into_platform(&TILTED_EXAMPLE_INPUT)
        );
    }

    #[test]
    fn tilt_1_cycle() {
        assert_eq!(
            super::tilt_one_cycle(&super::parse_into_platform(&EXAMPLE_INPUT)),
            super::parse_into_platform(&EXAMPLE_INPUT_AFTER_1_CYCLE)
        );
    }

    #[test]
    fn get_total_load() {
        assert_eq!(super::get_total_load(&get_parsed_small_input()), 5);
    }
}
