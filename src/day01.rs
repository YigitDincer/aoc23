fn solve_for_n_zeroes(input: &str, n: usize) -> usize {
    let prefix = "0".repeat(n);

    for i in 1.. {
        let modified_input = format!("{input}{i}");
        let digest = md5::compute(modified_input);
        let hex_string = format!("{digest:x}");
        if hex_string.starts_with(&prefix) {
            return i;
        }
    }

    unreachable!()
}

pub fn solve(input: &str) {
    println!("{}", solve_for_n_zeroes(input, 5));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(solve_for_n_zeroes("abcdef", 5), 609043);
    }
}
