fn get_vertical_symmetrical_index(input: &str, discrepancy: usize) -> Option<usize> {
    let col_cnt = input.lines().next().unwrap().len();

    (0..col_cnt - 1)
        .find(|&i| {
            (0..std::cmp::min(i + 1, col_cnt - i - 1))
                .map(|offset_from_mirror| {
                    input
                        .lines()
                        .map(|line| {
                            (line.as_bytes()[i - offset_from_mirror]
                                != line.as_bytes()[i + 1 + offset_from_mirror])
                                as usize
                        })
                        .sum::<usize>()
                })
                .sum::<usize>()
                == discrepancy
        })
        .map(|x| x + 1)
}

fn get_horizontal_symmetrical_index(input: &str, discrepancy: usize) -> Option<usize> {
    let row_cnt = input.lines().count();
    let col_cnt = input.lines().next().unwrap().len();

    (0..row_cnt - 1)
        .find(|&i| {
            (0..std::cmp::min(i + 1, row_cnt - i - 1))
                .map(|offset_from_mirror| {
                    (0..col_cnt)
                        .filter(|col_idx| {
                            input
                                .lines()
                                .nth(i - offset_from_mirror)
                                .unwrap()
                                .as_bytes()[*col_idx]
                                != input
                                    .lines()
                                    .nth(i + 1 + offset_from_mirror)
                                    .unwrap()
                                    .as_bytes()[*col_idx]
                        })
                        .count()
                })
                .sum::<usize>()
                == discrepancy
        })
        .map(|x| x + 1)
}

fn summarize(puzzle: &str, discrepancy: usize) -> usize {
    get_vertical_symmetrical_index(&puzzle, discrepancy)
        .or_else(|| get_horizontal_symmetrical_index(&puzzle, discrepancy).map(|x| x * 100))
        .unwrap_or(0)
}

pub fn solve(input: &str) {
    println!(
        "{}",
        input
            .split("\n\n")
            .map(|puzzle| summarize(puzzle, 0))
            .sum::<usize>()
    );

    println!(
        "{}",
        input
            .split("\n\n")
            .map(|puzzle| summarize(puzzle, 1))
            .sum::<usize>()
    );
}

#[cfg(test)]
mod tests {

    const VERTICAL_SYM_INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";

    const HORIZONTAL_SYM_INPUT: &str = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn vertical_symmetry() {
        assert_eq!(super::summarize(&VERTICAL_SYM_INPUT, 0), 5);
    }

    #[test]
    fn summarize_wo_discrepancy() {
        assert_eq!(super::summarize(&HORIZONTAL_SYM_INPUT, 0), 400);
    }

    #[test]
    fn summarize_with_discrepancy() {
        assert_eq!(super::summarize(&VERTICAL_SYM_INPUT, 1), 300);
    }
}
