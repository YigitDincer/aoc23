use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
enum Tile {
    Rock,
    Plot,
    S,
}

fn parse(input: &str) -> HashMap<(i32, i32), Tile>
{
    input.lines().enumerate().flat_map(|(y, line)|
        line.chars().enumerate().map(move |(x, ch)|
            match ch{
                '.' => ((x as i32, y as i32), Tile::Plot),
                '#' => ((x as i32, y as i32), Tile::Rock),
                'S' => ((x as i32, y as i32), Tile::S),
                _ => panic!("Unknown tile: {}", ch),
            }
        )
    ).collect()
}

fn get_neighbors((x, y): (i32, i32)) -> Vec<(i32, i32)>
{
    vec![(x-1, y), (x+1, y), (x, y-1), (x, y+1)]
}

fn get_valid_neigbors((x, y): (i32, i32), map: &HashMap<(i32, i32), Tile>) -> Vec<(i32, i32)>
{
    get_neighbors((x, y)).into_iter().filter(|pos| map.get(pos) != Some(&Tile::Rock)).collect()
}

fn install_rock_to_the_edges(map: &HashMap<(i32, i32), Tile>) -> HashMap<(i32, i32), Tile>
{
    let mut cloned_map = map.clone();

    let min_x = cloned_map.keys().map(|(x, _)| x).min().unwrap();
    let max_x = cloned_map.keys().map(|(x, _)| x).max().unwrap();
    let min_y = cloned_map.keys().map(|(_, y)| y).min().unwrap();
    let max_y = cloned_map.keys().map(|(_, y)| y).max().unwrap();

    let min_rock_x = *min_x - 1;
    let max_rock_x = *max_x + 1;
    let min_rock_y = *min_y - 1;
    let max_rock_y = *max_y + 1;

    for x in min_rock_x..=max_rock_x {
        cloned_map.insert((x, min_rock_y), Tile::Rock);
        cloned_map.insert((x, max_rock_y), Tile::Rock);
    }
    for y in min_rock_y..=max_rock_y {
        cloned_map.insert((min_rock_x, y), Tile::Rock);
        cloned_map.insert((max_rock_x, y), Tile::Rock);
    }

    cloned_map
}

fn get_next(map : &HashMap<(i32, i32), Tile>, current_pos : &HashSet<(i32, i32)>) -> HashSet<(i32, i32)>
{
    current_pos.iter().flat_map(|&pos| get_valid_neigbors(pos, map)).collect::<HashSet<(i32, i32)>>()
}

fn solve_1(map : &HashMap<(i32, i32), Tile>, steps: usize) -> usize
{
    let starting_pos = map.iter().find(|(_, &tile)| tile == Tile::S).unwrap().0;
    let map_with_border = install_rock_to_the_edges(map);

    let mut reached_pos = HashSet::new();
    reached_pos.insert(*starting_pos);

    for _ in 0..steps {
        reached_pos = get_next(&map_with_border, &reached_pos);
    }    

    reached_pos.len()
}



pub fn solve(input: &str) {
    println!("{}", solve_1(&parse(input), 64));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    fn get_example_input() -> HashMap<(i32, i32), Tile>
    {
        let mut map = HashMap::new();
        map.insert(( 0,0), Tile::Plot);
        map.insert(( 1,0), Tile::Plot);
        map.insert(( 2,0), Tile::Plot);
        map.insert(( 3,0), Tile::Plot);
        map.insert(( 4,0), Tile::Plot);
        map.insert(( 5,0), Tile::Plot);
        map.insert(( 6,0), Tile::Plot);
        map.insert(( 7,0), Tile::Plot);
        map.insert(( 8,0), Tile::Plot);
        map.insert(( 9,0), Tile::Plot);
        map.insert(( 10,0), Tile::Plot);
        map.insert(( 0,1), Tile::Plot);
        map.insert(( 1,1), Tile::Plot);
        map.insert(( 2,1), Tile::Plot);
        map.insert(( 3,1), Tile::Plot);
        map.insert(( 4,1), Tile::Plot);
        map.insert(( 5,1), Tile::Rock);
        map.insert(( 6,1), Tile::Rock);
        map.insert(( 7,1), Tile::Rock);
        map.insert(( 8,1), Tile::Plot);
        map.insert(( 9,1), Tile::Rock);
        map.insert(( 10,1), Tile::Plot);
        map
    }

    const SMALL_INPUT: &str = "..#
.##";

fn get_small_input_with_installed_rocks() -> HashMap<(i32, i32), Tile>{
    let mut map = HashMap::new();

    map.insert((-1,-1), Tile::Rock);
    map.insert(( 0,-1), Tile::Rock);
    map.insert(( 1,-1), Tile::Rock);
    map.insert(( 2,-1), Tile::Rock);
    map.insert(( 3,-1), Tile::Rock);

    map.insert((-1, 0), Tile::Rock);
    map.insert(( 0, 0), Tile::Plot);
    map.insert(( 1, 0), Tile::Plot);
    map.insert(( 2, 0), Tile::Rock);
    map.insert(( 3, 0), Tile::Rock);

    map.insert((-1, 1), Tile::Rock);
    map.insert(( 0, 1), Tile::Plot);
    map.insert(( 1, 1), Tile::Rock);
    map.insert(( 2, 1), Tile::Rock);
    map.insert(( 3, 1), Tile::Rock);

    map.insert((-1, 2), Tile::Rock);
    map.insert(( 0, 2), Tile::Rock);
    map.insert(( 1, 2), Tile::Rock);
    map.insert(( 2, 2), Tile::Rock);
    map.insert(( 3, 2), Tile::Rock);

    map
}

#[test]
fn parse() {
    assert_eq!(super::parse(&EXAMPLE_INPUT.lines().take(2).collect::<Vec<_>>().join("\n")), get_example_input());
}

#[test]
fn install_rocks() {

    assert_eq!(install_rock_to_the_edges(&super::parse(&SMALL_INPUT)), get_small_input_with_installed_rocks());
}

#[test]
fn solve_1() {
    assert_eq!(super::solve_1(&super::parse(&EXAMPLE_INPUT), 6), 16);
}
    
}
