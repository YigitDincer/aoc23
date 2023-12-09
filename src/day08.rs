use std::collections::HashMap;

type Network = HashMap<String, (String, String)>;

fn parse_into_network(input: &str) -> Network {
    let mut nodes: Network = HashMap::new();

    input.lines().for_each(|line| {
        let (node_name, rest) = line.split_once(" = ").unwrap();
        let (left, right) = rest.split_once(", ").unwrap();
        nodes.insert(
            node_name.into(),
            (
                left.trim_start_matches("(").into(),
                right.trim_end_matches(")").into(),
            ),
        );
    });

    nodes
}

fn parse_directions(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn solve_1(network: Network, directions: Vec<char>) -> usize {
    let mut current_node_name = "AAA";
    let mut ctr = 0;

    while current_node_name != "ZZZ" {
        let (l, r) = network.get(current_node_name).unwrap();
        current_node_name = if directions[ctr % directions.len()] == 'R' {
            r
        } else {
            l
        };
        ctr += 1;
    }

    ctr
}

pub fn solve(input: &str) {
    const DIRECTIONS : &str = "LRLRRRLRRLRRRLRRRLLLLLRRRLRLRRLRLRLRRLRRLRRRLRLRLRRLLRLRRLRRLRRLRRRLLRRRLRRRLRRLRLLLRRLRRRLRLRRLRRRLRRLRLLLRRRLRRLRRLRRRLRRRLRRRLRLRLRLRRRLRRRLLLRRLLRRRLRLRLRRRLRRRLRRLRRRLRLRLLRRRLRLRRLRLRLRRLLLRRRLRRRLRRLRRLRLRRLLRRLRRRLRRRLLRRRLRRLRLLRRLRLRRLLRRRLLLLRRLRRRLRLRRLLRLLRRRLLRRLLRRRLRRRLRRLLRLRLLRRLLRLLLRRRR";
    println!(
        "{}",
        solve_1(parse_into_network(&input), parse_directions(DIRECTIONS))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_NETWORK: &str = "AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const EXAMPLE_DIRECTIONS: &str = "RL";

    fn get_example_network_as_nodes() -> Network {
        let mut network: Network = HashMap::new();
        network.insert("AAA".into(), ("BBB".into(), "CCC".into()));
        network.insert("BBB".into(), ("DDD".into(), "EEE".into()));
        network.insert("CCC".into(), ("ZZZ".into(), "GGG".into()));
        network.insert("DDD".into(), ("DDD".into(), "DDD".into()));
        network.insert("EEE".into(), ("EEE".into(), "EEE".into()));
        network.insert("GGG".into(), ("GGG".into(), "GGG".into()));
        network.insert("ZZZ".into(), ("ZZZ".into(), "ZZZ".into()));
        network
    }

    #[test]
    fn parse() {
        assert_eq!(
            parse_into_network(&EXAMPLE_NETWORK),
            get_example_network_as_nodes()
        );
        assert_eq!(super::parse_directions(&EXAMPLE_DIRECTIONS), vec!['R', 'L']);
    }
}
