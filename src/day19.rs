use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Operator {
    LessThan,
    GreaterThan,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Rule {
    operator: Option<Operator>,
    reference: Option<i32>,
    element: Option<String>,
    target: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Workflow {
    rules: Vec<Rule>,
}

type Workflows = HashMap<String, Workflow>;

impl Part {
    fn get(&self, element: &str) -> i32 {
        match element {
            "x" => self.x,
            "m" => self.m,
            "a" => self.a,
            "s" => self.s,
            _ => panic!("Unknown element {}", element),
        }
    }
}

fn apply_operator(rule: &Rule, part: &Part) -> bool {
    let Some(element) = rule.element.as_ref() else {
        return true;
    };

    match rule.operator {
        Some(Operator::LessThan) => part.get(&element) < rule.reference.unwrap(),
        Some(Operator::GreaterThan) => part.get(&element) > rule.reference.unwrap(),
        None => true,
    }
}

fn do_workflow(workflows: &Workflows, part: &Part, current_workflow_name: &str) -> bool {
    let current_workflow = workflows.get(current_workflow_name).unwrap();

    for rule in &current_workflow.rules {
        if apply_operator(&rule, &part) {
            let target_name = rule.target.as_str();
            return match target_name {
                "A" => true,
                "R" => false,
                _ => do_workflow(workflows, part, target_name),
            };
        } else {
            continue;
        }
    }

    unreachable!()
}

fn parse(input: &str) -> (HashMap<String, Workflow>, Vec<Part>) {
    let mut workflows = HashMap::new();
    let mut parts = vec![];

    for line in input.lines() {
        if line.starts_with('{') {
            let part = line
                .trim_start_matches('{')
                .trim_end_matches('}')
                .split(',')
                .map(|pair| {
                    let mut iter = pair.split('=');
                    let key = iter.next().unwrap().trim();
                    let value = iter.next().unwrap().trim().parse().unwrap();
                    (key, value)
                })
                .fold(
                    Part {
                        x: 0,
                        m: 0,
                        a: 0,
                        s: 0,
                    },
                    |mut part, (key, value)| {
                        match key {
                            "x" => part.x = value,
                            "m" => part.m = value,
                            "a" => part.a = value,
                            "s" => part.s = value,
                            _ => (),
                        }
                        part
                    },
                );

            parts.push(part);
        } else if let Some(_) = line.chars().next() {
            let mut iter = line.split('{');
            let name = iter.next().unwrap().trim();
            let rules = iter.next().unwrap().trim_end_matches('}');

            workflows.insert(
                name.to_string(),
                Workflow {
                    rules: rules
                        .split(',')
                        .map(|rule| match rule.split_once(':') {
                            Some((condition, target)) => {
                                let mut iter = condition.chars();
                                let element = iter.next().map(|c| c.to_string());
                                let operator = match iter.next() {
                                    Some('<') => Some(Operator::LessThan),
                                    Some('>') => Some(Operator::GreaterThan),
                                    _ => None,
                                };
                                let reference = iter.as_str().parse().ok();

                                Rule {
                                    operator,
                                    reference,
                                    element,
                                    target: target.to_string(),
                                }
                            }
                            None => Rule {
                                operator: None,
                                reference: None,
                                element: None,
                                target: rule.to_string(),
                            },
                        })
                        .collect(),
                },
            );
        }
    }

    (workflows, parts)
}

pub fn solve(input: &str) {
    let (workflows, parts) = parse(input);
    println!(
        "{}",
        parts
            .iter()
            .filter(|part| do_workflow(&workflows, part, "in"))
            .map(|Part { x, m, a, s }| x + m + a + s)
            .sum::<i32>()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn parse() {
        assert_eq!(
            super::parse(&EXAMPLE_INPUT.lines().next().unwrap()),
            (
                HashMap::from_iter(vec![(
                    "px".to_string(),
                    Workflow {
                        rules: vec![
                            Rule {
                                operator: Some(Operator::LessThan),
                                reference: Some(2006),
                                element: Some("a".to_string()),
                                target: "qkq".to_string(),
                            },
                            Rule {
                                operator: Some(Operator::GreaterThan),
                                reference: Some(2090),
                                element: Some("m".to_string()),
                                target: "A".to_string(),
                            },
                            Rule {
                                operator: None,
                                reference: None,
                                element: None,
                                target: "rfg".to_string(),
                            },
                        ]
                    }
                )]),
                Vec::new()
            )
        )
    }

    #[test]
    fn parse_2() {
        assert_eq!(
            super::parse(&EXAMPLE_INPUT.lines().last().unwrap()),
            (
                HashMap::new(),
                vec![Part {
                    x: 2127,
                    m: 1623,
                    a: 2188,
                    s: 1013,
                }]
            )
        );
    }

    #[test]
    fn do_workflow() {
        let parsed_input = super::parse(&EXAMPLE_INPUT);
        assert_eq!(
            super::do_workflow(&parsed_input.0, &parsed_input.1.first().unwrap(), "in"),
            true
        );
    }
}
