use std::collections::HashMap;

type Boxes = HashMap<usize, Vec<(String, Option<usize>)>>;

fn get_hash(input: &str) -> usize {
    input
        .chars()
        .fold(0, |acc, c| ((acc + c as usize) * 17) % 256)
}

#[derive(Debug, Eq, PartialEq)]
struct Operation {
    operator: String,
    label: String,
    hash: usize,
    val: Option<usize>,
}

fn parse_operations(input: &str) -> Vec<Operation> {
    input
        .split(",")
        .map(|op| {
            let (label, value) = op.split_once(|c| c == '=' || c == '-').unwrap();
            Operation {
                operator: if value.is_empty() { '-' } else { '=' }.to_string(),
                label: label.to_string(),
                hash: get_hash(label),
                val: if value.is_empty() {
                    None
                } else {
                    Some(value.parse::<usize>().unwrap())
                },
            }
        })
        .collect()
}

fn calculate_focusing_power(boxes: &Boxes) -> usize {
    boxes
        .iter()
        .map(|(hash, content)| {
            content
                .iter()
                .enumerate()
                .map(|(slot, (_, val))| (hash + 1) * (slot + 1) * val.unwrap())
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn get_boxes(input: &str) -> Boxes {
    let mut boxes: Boxes = HashMap::new();

    parse_operations(input).iter().for_each(|op| {
        let box_val = boxes.entry(op.hash).or_insert_with(Vec::new);

        match op.operator.as_str() {
            "=" => {
                if let Some((_, val)) = box_val
                    .iter_mut()
                    .find(|(label, val)| label == op.label.as_str())
                {
                    *val = op.val;
                } else {
                    box_val.push((op.label.clone(), op.val));
                }
            }
            "-" => {
                box_val.retain(|(label, _)| label != op.label.as_str());
            }
            _ => panic!("Unknown operation"),
        }
    });

    boxes
}

pub fn solve(input: &str) {
    println!("{}", input.split(",").map(|a| get_hash(a)).sum::<usize>());
    print!("{}", calculate_focusing_power(&get_boxes(input)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_hash() {
        assert_eq!(super::get_hash("rn=1"), 30);
    }

    #[test]
    fn get_hash_2() {
        assert_eq!(super::get_hash("rn"), 0);
    }

    #[test]
    fn parse_operations() {
        assert_eq!(
            super::parse_operations("rn=2"),
            vec![super::Operation {
                operator: "=".to_string(),
                label: "rn".to_string(),
                hash: 0,
                val: Some(2),
            }]
        );

        assert_eq!(
            super::parse_operations("cm-"),
            vec![super::Operation {
                operator: "-".to_string(),
                label: "cm".to_string(),
                hash: 0,
                val: None
            }]
        );
    }

    #[test]
    fn calculate_focusing_power() {
        assert_eq!(
            super::calculate_focusing_power(&super::get_boxes(
                "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"
            )),
            145
        );
    }
}
