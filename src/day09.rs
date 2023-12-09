fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|f| f.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn extrapolate_end(report: &[i64]) -> i64 {
    if report.iter().any(|&a| a != 0) {
        let differences: Vec<i64> = report
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect();
        return extrapolate_end(&differences) + report.last().unwrap();
    } else {
        return 0;
    }
}

fn extrapolate_begin(report: &[i64]) -> i64 {
    if report.iter().any(|&a| a != 0) {
        let differences: Vec<i64> = report
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect();
        return report.first().unwrap() - extrapolate_begin(&differences);
    } else {
        return 0;
    }
}

fn solve_for(input: &str, extrapolation: fn(&[i64]) -> i64) -> i64 {
    parse(input)
        .iter()
        .map(|report| extrapolation(report))
        .sum()
}

pub fn solve(input: &str) {
    println!("{}", solve_for(&input, extrapolate_end));
    println!("{}", solve_for(&input, extrapolate_begin));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        assert_eq!(
            super::parse("0 3 6 9 12 15").first().unwrap(),
            &[0, 3, 6, 9, 12, 15]
        );
    }

    #[test]
    fn extrapolate() {
        assert_eq!(super::extrapolate_end(&[0, 3, 6, 9, 12, 15]), 18);
    }
}
