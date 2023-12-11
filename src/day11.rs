use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Star,
    EmptySpace,
}

type Universe = Vec<Vec<Tile>>;

fn parse_input(input: &str) -> Universe {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '.' => Tile::EmptySpace,
                    '#' => Tile::Star,
                    _ => panic!("unknown tile type"),
                })
                .collect()
        })
        .collect()
}

struct DarkMatter {
    rows: Vec<usize>,
    columns: Vec<usize>,
}

fn get_dark_matter(universe: &Universe) -> DarkMatter {
    let rows = universe
        .iter()
        .positions(|row| row.iter().all(|tile| tile == &Tile::EmptySpace))
        .collect();

    let columns = (0..universe[0].len())
        .filter(|column_idx| {
            universe
                .iter()
                .all(|row| row[*column_idx] == Tile::EmptySpace)
        })
        .collect();

    DarkMatter { rows, columns }
}

fn get_star_coordinates(universe: &Universe) -> Vec<(usize, usize)> {
    universe
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.iter()
                .positions(|tile| tile == &Tile::Star)
                .map(move |column_idx| (row_idx, column_idx))
        })
        .collect()
}

fn get_manhattan_distance(
    star_1: (usize, usize),
    star_2: (usize, usize),
    dark_matter: &DarkMatter,
    expansion_rate: usize,
) -> usize {
    let min_row = star_1.0.min(star_2.0);
    let max_row = star_1.0.max(star_2.0);
    let min_col = star_1.1.min(star_2.1);
    let max_col = star_1.1.max(star_2.1);

    let expanded_rows = dark_matter
        .rows
        .iter()
        .filter(|row| (min_row..max_row).contains(row))
        .count();
    let expanded_cols = dark_matter
        .columns
        .iter()
        .filter(|col| (min_col..max_col).contains(col))
        .count();

    max_row - min_row + max_col - min_col + (expanded_rows + expanded_cols) * (expansion_rate - 1)
}

fn solve_for(input: &str, expansion_rate: usize) -> usize {
    let universe = parse_input(input);

    let dark_matter = get_dark_matter(&universe);
    let stars = get_star_coordinates(&universe);

    stars
        .iter()
        .tuple_combinations()
        .map(|(&star_1, &star_2)| {
            get_manhattan_distance(star_1, star_2, &dark_matter, expansion_rate)
        })
        .sum()
}

pub fn solve(input: &str) {
    println!("{}", solve_for(&input, 2));
    println!("{}", solve_for(&input, 1000000));
}

#[cfg(test)]
mod tests {
    use super::*;
}
