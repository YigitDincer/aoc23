fn calculate_possibilities(time: u64, distance: u64) -> usize {
    (1..time).filter(|i| i * (time - i) > distance).count()
}

pub fn solve(input: &str) {
    println!(
        "{}",
        calculate_possibilities(51, 222)
            * calculate_possibilities(92, 2031)
            * calculate_possibilities(68, 1126)
            * calculate_possibilities(90, 1225)
    );

    println!("{}", calculate_possibilities(51926890, 222203111261225));
}
