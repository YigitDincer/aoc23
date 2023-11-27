fn parse(input: &str) -> Vec<i32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn add_numbers(input: Vec<i32>) -> i32 {
    input.iter().sum()
}

pub fn solve(input: &str) {
    println!("{}", add_numbers(parse(input)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(add_numbers(vec![1, 2, 3]), 6);
    }

    #[test]
    fn parse() {
        assert_eq!(super::parse(EXAMPLE_INPUTS), vec![1, 2, 3])
    }

    const EXAMPLE_INPUTS: &str = r#"1
2
3
"#;
}
