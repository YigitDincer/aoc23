use itertools::Itertools;

#[derive(Debug, Clone)]
struct Hail {
    x: f64,
    y: f64,
    _z: f64,
    vx: f64,
    vy: f64,
    _vz: f64,
}

fn parse(input: &str) -> Vec<Hail> {
    input
        .lines()
        .map(|line| line.split_once('@').unwrap())
        .map(|(pos, vel)| {
            let mut pos_split = pos.split(',');
            let mut vel_split = vel.split(',');

            Hail {
                x: pos_split.next().unwrap().trim().parse().unwrap(),
                y: pos_split.next().unwrap().trim().parse().unwrap(),
                _z: pos_split.next().unwrap().trim().parse().unwrap(),

                vx: vel_split.next().unwrap().trim().parse().unwrap(),
                vy: vel_split.next().unwrap().trim().parse().unwrap(),
                _vz: vel_split.next().unwrap().trim().parse().unwrap(),
            }
        })
        .collect()
}

fn calculate_constant_2d(hail: &Hail) -> f64 {
    hail.y - (hail.vy / hail.vx) * hail.x
}

fn calculate_intersection_2d(hail_1: &Hail, hail_2: &Hail) -> Option<(f64, f64)> {
    let c1 = calculate_constant_2d(hail_1);
    let c2 = calculate_constant_2d(hail_2);
    let m1 = hail_1.vy / hail_1.vx;
    let m2 = hail_2.vy / hail_2.vx;

    if m1 == m2 {
        return None;
    }

    let (intersect_x, intersect_y) = ((c2 - c1) / (m1 - m2), (m2 * c1 - m1 * c2) / (m2 - m1));

    if (intersect_x - hail_1.x) / hail_1.vx > 0.0 && (intersect_x - hail_2.x) / hail_2.vx > 0.0 {
        Some((intersect_x, intersect_y))
    } else {
        None
    }
}

fn solve_1(input: &str, limits: &(f64, f64)) -> usize {
    parse(input)
        .into_iter()
        .tuple_combinations()
        .filter(|(hail_1, hail_2)| {
            if let Some((x, y)) = calculate_intersection_2d(&hail_1, &hail_2) {
                x >= limits.0 && x <= limits.1 && y >= limits.0 && y <= limits.1
            } else {
                false
            }
        })
        .count()
}

pub fn solve(input: &str) {
    println!(
        "{}",
        solve_1(&input, &(200000000000000.0, 400000000000000.0))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    #[test]
    fn example() {
        assert_eq!(solve_1(&EXAMPLE_INPUT, &(7.0, 27.0)), 2);
    }
}
