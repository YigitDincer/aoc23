use std::{collections::{HashMap, HashSet}, vec};

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
enum Tile{
    Empty,
    MirrorRight,
    MirrorLeft,
    SplitterVertical,
    SplitterHorizontal,
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash, PartialOrd, Ord)]
enum Direction{
    Up,
    Down,
    Left,
    Right,
}

type Facility = HashMap<(i32, i32), Tile>;

fn parse(input: &str) -> Facility{
    let mut map = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let tile = match c {
                '.' => Tile::Empty,
                '/' => Tile::MirrorRight,
                '\\' => Tile::MirrorLeft,
                '|' => Tile::SplitterVertical,
                '-' => Tile::SplitterHorizontal,
                _ => panic!("Unknown tile: {}", c),
            };
            map.insert((x as i32, y as i32), tile);
        }
    }

    map
}

fn next_for_pos(facility: &Facility, current_pos: (i32, i32, Direction)) -> Vec<(i32, i32, Direction)> {
    let (x, y, current_dir) = current_pos;

    match current_dir {
        Direction::Up => {
            match facility.get(&(x, y - 1)) {
                Some(Tile::Empty) => vec![(x, y-1, Direction::Up)],
                Some(Tile::MirrorRight) => vec![(x, y-1, Direction::Right)],
                Some(Tile::MirrorLeft) => vec![(x, y-1, Direction::Left)],
                Some(Tile::SplitterVertical) => vec![(x, y-1, Direction::Up)],
                Some(Tile::SplitterHorizontal) => vec![(x, y-1, Direction::Right), (x, y-1, Direction::Left)],
                _ => vec![],
            }
        }
        Direction::Down => {
            match facility.get(&(x, y + 1)) {
                Some(Tile::Empty) => vec![(x, y+1,Direction::Down)],
                Some(Tile::MirrorRight) => vec![(x, y+1, Direction::Left)],
                Some(Tile::MirrorLeft) => vec![(x, y+1, Direction::Right)],
                Some(Tile::SplitterVertical) => vec![(x, y+1,Direction::Down)],
                Some(Tile::SplitterHorizontal) => vec![(x, y+1,Direction::Right), (x, y+1, Direction::Left)],
                _ => vec![],
            }
        }
        Direction::Left => {
            match facility.get(&(x - 1, y)) {
                Some(Tile::Empty) => vec![(x-1, y,  Direction::Left)],
                Some(Tile::MirrorRight) => vec![(x-1, y,  Direction::Down)],
                Some(Tile::MirrorLeft) => vec![(x-1, y,  Direction::Up)],
                Some(Tile::SplitterVertical) => vec![(x-1, y,  Direction::Down), (x-1, y,  Direction::Up)],
                Some(Tile::SplitterHorizontal) => vec![(x-1, y,  Direction::Left)],
                _ => vec![],
            }
        }
        Direction::Right => {
            match facility.get(&(x + 1, y)) {
                Some(Tile::Empty) => vec![(x+1, y,  Direction::Right)],
                Some(Tile::MirrorRight) => vec![(x+1, y,  Direction::Up)],
                Some(Tile::MirrorLeft) => vec![(x+1, y,  Direction::Down)],
                Some(Tile::SplitterVertical) => vec![(x+1, y,  Direction::Down), (x+1, y,  Direction::Up)],
                Some(Tile::SplitterHorizontal) => vec![(x+1, y,  Direction::Right)],
                _ => vec![],
            }
        }
    }

    
}

fn get_next_all(facility: &Facility, current_pos: HashSet<(i32, i32, Direction)>) -> HashSet<(i32, i32, Direction)> {
    current_pos.iter().flat_map(|&(x, y, dir)| next_for_pos(facility, (x, y, dir))).collect::<HashSet<(i32, i32, Direction)>>()
}

fn solve_1(facility: &Facility, entry_point: (i32, i32), direction: Direction) -> usize {
    let mut beam_pos = HashSet::new();
    let (x, y) = entry_point;
    beam_pos.insert((x, y, direction));
    
    let mut visited_pos = HashSet::new();

    loop
    {
        beam_pos = beam_pos.difference(&visited_pos).cloned().collect::<HashSet<(i32, i32, Direction)>>();
        visited_pos.extend(beam_pos.iter().cloned());

        beam_pos = get_next_all(facility, beam_pos);
        if beam_pos.len() == 0 {
            break;
        }
    }

    let unique_pos : HashSet<_> = visited_pos.into_iter().map(|(x,y, _)| (x,y)).collect();
    unique_pos.len() - 1
}

fn solve_2(facility: &Facility) -> usize
{
    let width = *facility.iter().map(|((x, _), _)| x).max().unwrap();
    let height = *facility.iter().map(|((_, y), _)| y).max().unwrap();

    let right_max = (0..height).into_iter().map(|y| solve_1(facility, (-1, y), Direction::Right)).max();
    let left_max = (0..height).into_iter().map(|y| solve_1(facility, (width, y), Direction::Left)).max();
    let down_max = (0..width).into_iter().map(|x| solve_1(facility, (x, -1), Direction::Down)).max();
    let up_max = (0..width).into_iter().map(|x| solve_1(facility, (x, height), Direction::Up)).max();

    right_max.max(left_max).max(down_max).max(up_max).unwrap()
}

pub fn solve(input: &str) {
    println!("{}", solve_1(&parse(&input), (-1, 0), Direction::Right));
    println!("{}", solve_2(&parse(&input)));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

#[test]
fn solve_1() {
    assert_eq!(super::solve_1(&parse(&EXAMPLE_INPUT), (-1, 0), Direction::Right), 46);
}

#[test]
fn solve_2(){
    assert_eq!(super::solve_2(&parse(&EXAMPLE_INPUT)), 51);
}


}
