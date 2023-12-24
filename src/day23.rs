use std::{collections::HashMap, vec};

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
enum Tile {
    UpSlope,
    DownSlope,
    LeftSlope,
    RightSlope,
    Path,
    Forest,
}

type Trail = HashMap<(i32, i32), Tile>;

fn parse(input: &str) -> Trail {
    input.lines().enumerate().flat_map(|(y, line)| {
        line.chars().enumerate().map(move |(x, c)| {
            ((x as i32, y as i32),
            match c {
                '.' => Tile::Path,
                '#' => Tile::Forest,
                '>' => Tile::RightSlope,
                '<' => Tile::LeftSlope,
                '^' => Tile::UpSlope,
                'v' => Tile::DownSlope,
                _ => panic!("Unknown tile: {}", c),
            })
        })
    }).collect()

}

fn parse_for_part2(input: &str) -> Trail {
    input.lines().enumerate().flat_map(|(y, line)| {
        line.chars().enumerate().map(move |(x, c)| {
            ((x as i32, y as i32),
            match c {
                '.' => Tile::Path,
                '#' => Tile::Forest,
                '>' => Tile::Path,
                '<' => Tile::Path,
                '^' => Tile::Path,
                'v' => Tile::Path,
                _ => panic!("Unknown tile: {}", c),
            })
        })
    }).collect()

}


fn get_valid_neighbors(trail: &Trail, pos: (i32, i32)) -> Vec<(i32, i32)> {
    let (x, y) = pos;

    let mut valid_next_steps = vec![];

    let up_trail = trail.get(&(x, y-1)).unwrap();
    let down_trail = trail.get(&(x, y+1)).unwrap();
    let left_trail = trail.get(&(x-1, y)).unwrap();
    let right_trail = trail.get(&(x+1, y)).unwrap();

    if *up_trail != Tile::DownSlope && *up_trail != Tile::Forest {
        valid_next_steps.push((x, y-1));
    }

    if *down_trail != Tile::UpSlope && *down_trail != Tile::Forest {
        valid_next_steps.push((x, y+1));
    }

    if *left_trail != Tile::RightSlope && *left_trail != Tile::Forest {
        valid_next_steps.push((x-1, y));
    }

    if *right_trail != Tile::LeftSlope && *right_trail != Tile::Forest {
        valid_next_steps.push((x+1, y));
    }

    valid_next_steps
}


fn solve_1(trail: &Trail) -> usize
{
    let mut modified_trail = trail.clone();
    let max_x = *modified_trail.iter().map(|((x, _), _)| x).max().unwrap();
    let max_y = *modified_trail.iter().map(|((_, y), _)| y).max().unwrap();

    modified_trail.insert((1, 0), Tile::Forest);
    modified_trail.insert((max_x-1, max_y), Tile::Forest);

    let starting_point = (1,1);
    let ending_point = (max_x-1, max_y-1);

    let mut longest_route = 0;

    let mut unfinished_routes = vec![vec![starting_point]];

    while !&unfinished_routes.is_empty() {
        let current_routes = unfinished_routes.clone();
        let entry_count = current_routes.len();

        for _ in 0..entry_count {
            let mut route = unfinished_routes.pop().unwrap();
            let last_pos = route.last().unwrap();
            let valid_neighbors = get_valid_neighbors(&modified_trail, *last_pos);

            if valid_neighbors.contains(&ending_point) {
                route.push(ending_point);
                longest_route = longest_route.max(route.len());
                continue;
            }

            for neighbor in valid_neighbors {
                if !route.contains(&neighbor) {
                    let mut new_route = route.clone();
                    new_route.push(neighbor);
                    unfinished_routes.push(new_route);
                }
            }
        }
    }


    longest_route + 1
}

pub fn solve(input: &str) {
    println!("Part 1: {}", solve_1(&parse(&input)));
    println!("Part 2: {}", solve_1(&parse_for_part2(&input)));
}

#[cfg(test)]
mod tests {
    use super::*;

const EXAMPLE_INPUT: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

#[test]
fn solve_1() {
    assert_eq!(super::solve_1(&parse(&EXAMPLE_INPUT)), 94);
}

#[test]
fn solve_2() {
    assert_eq!(super::solve_1(&parse_for_part2(&EXAMPLE_INPUT)), 154);
}


}
