fn solve_1(input: &str) -> u32 {
    input.lines().map(extract_first_and_last_digits).sum()
}

fn extract_first_and_last_digits(input: &str) -> u32 {
    let v: Vec<_> = input.chars().filter_map(|a| a.to_digit(10)).collect();
    v.first().unwrap() * 10 + v.last().unwrap()
}

fn solve_2(input: &str) -> u32 {
    input
        .lines()
        .map(replace_text)
        .map(|a| extract_first_and_last_digits(a.as_str()))
        .sum()
}

fn replace_text(input: &str) -> String {
    input
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine")
}

pub fn solve(input: &str) {
    println!("{}", solve_1(input));
    println!("{}", solve_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(extract_first_and_last_digits("1abc2"), 12);
        assert_eq!(extract_first_and_last_digits("a1b2c3d4e5f"), 15);
    }

    #[test]
    fn example_part_2() {
        assert_eq!(solve_2("zoneight"), 18);
        assert_eq!(solve_2("eighthree"), 83);
    }
}
