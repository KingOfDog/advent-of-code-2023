use core::panic;
use std::str::FromStr;

use cgmath::{vec2, vec3, Vector2, Vector3};
use itertools::Itertools;
use z3::ast::Ast;

advent_of_code::solution!(24);

// struct Hailstone {
//     pos: Vector3<f64>,
//     vel: Vector3<f64>,
// }

// fn check_xy_collision(a: &Hailstone, b: &Hailstone) -> Option<Vector2<f64>> {
//     let dx = b.pos.x - a.pos.x;
//     let dy = b.pos.y - a.pos.y;
//     let det = a.vel.x * b.vel.y - b.vel.x * a.vel.y;
//     let u = (dy * b.vel.x - dx * b.vel.y) / det;
//     let v = (dy * a.vel.x - dx * a.vel.y) / det;
//     if u < 0. || v < 0. {
//         return None;
//     }

//     let m0 = a.vel.y / a.vel.x;
//     let m1 = b.vel.y / b.vel.x;
//     let b0 = a.pos.y - m0 * a.pos.x;
//     let b1 = b.pos.y - m1 * b.pos.x;
//     let x = (b1 - b0) / (m0 - m1);
//     let y = m0 * x + b0;

//     Some(vec2(x, y))
// }

struct Hailstone {
    // pos: (T, T, T),
    pos: Vector3<i128>,
    // vel: (T, T, T),
    vel: Vector3<i128>,
}

impl FromStr for Hailstone {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pos, vel) = s.split('@').collect_tuple().unwrap();
        let (x, y, z) = pos
            .split(',')
            .map(|x| x.trim().parse().unwrap())
            .collect_tuple()
            .unwrap();
        let pos = vec3(x, y, z);

        let (x, y, z) = vel
            .split(',')
            .map(|x| x.trim().parse().unwrap())
            .collect_tuple()
            .unwrap();
        let vel = vec3(x, y, z);

        Ok(Self { pos, vel })
    }
}

impl Hailstone {
    fn to_line(&self) -> (i128, i128, i128) {
        let Vector3 { x: x1, y: y1, .. } = self.pos;
        let Vector3 { x: vx, y: vy, .. } = self.vel;
        let a = vy;
        let b = -vx;
        let c = vx * y1 - vy * x1;
        (a, b, c)
    }
}

fn intersection(
    (a1, b1, c1): (i128, i128, i128),
    (a2, b2, c2): (i128, i128, i128),
) -> Option<(i128, i128)> {
    if (a1 * b2 - a2 * b1) == 0 {
        return None;
    }
    let x = (b1 * c2 - b2 * c1) / (a1 * b2 - a2 * b1);
    let y = (c1 * a2 - c2 * a1) / (a1 * b2 - a2 * b1);

    Some((x, y))
}

pub fn part_one(input: &str) -> Option<u32> {
    let hailstones = input
        .lines()
        .map(|line| line.parse::<Hailstone>().unwrap())
        .collect_vec();

    const MIN: i128 = 200_000_000_000_000;
    const MAX: i128 = 400_000_000_000_000;

    let lines = hailstones.iter().map(|h| h.to_line()).collect::<Vec<_>>();
    let number_of_collisions = hailstones
        .iter()
        .enumerate()
        .map(|(i, hailstone)| {
            hailstones[i + 1..]
                .iter()
                .enumerate()
                .filter(|(j, other)| {
                    let line1 = lines[i];
                    let line2 = lines[i + 1 + *j];

                    if let Some((x, y)) = intersection(line1, line2) {
                        if i128::signum(x - hailstone.pos.x) != i128::signum(hailstone.vel.x) {
                            return false;
                        }
                        if i128::signum(x - other.pos.x) != i128::signum(other.vel.x) {
                            return false;
                        }
                        if i128::signum(y - hailstone.pos.y) != i128::signum(hailstone.vel.y) {
                            return false;
                        }
                        if i128::signum(y - other.pos.y) != i128::signum(other.vel.y) {
                            return false;
                        }

                        x >= MIN && x <= MAX && y >= MIN && y <= MAX
                    } else {
                        false
                    }
                })
                .count() as u32
        })
        .sum::<u32>();

    Some(number_of_collisions)
}

pub fn part_two(input: &str) -> Option<u64> {
    let hailstones = input
        .lines()
        .map(|line| line.parse::<Hailstone>().unwrap())
        .collect_vec();

    let mut cfg = z3::Config::new();
    cfg.set_proof_generation(true);
    let context = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&context);

    let x = z3::ast::Int::new_const(&context, "x");
    let y = z3::ast::Int::new_const(&context, "y");
    let z = z3::ast::Int::new_const(&context, "z");
    let vx = z3::ast::Int::new_const(&context, "vx");
    let vy = z3::ast::Int::new_const(&context, "vy");
    let vz = z3::ast::Int::new_const(&context, "vz");

    for (i, hs) in hailstones.iter().take(3).enumerate() {
        let a = z3::ast::Int::from_i64(&context, hs.pos.x as i64);
        let va = z3::ast::Int::from_i64(&context, hs.vel.x as i64);
        let b = z3::ast::Int::from_i64(&context, hs.pos.y as i64);
        let vb = z3::ast::Int::from_i64(&context, hs.vel.y as i64);
        let c = z3::ast::Int::from_i64(&context, hs.pos.z as i64);
        let vc = z3::ast::Int::from_i64(&context, hs.vel.z as i64);

        let t = z3::ast::Int::new_const(&context, format!("t{i}"));
        solver.assert(&t.gt(&z3::ast::Int::from_i64(&context, 0)));
        solver.assert(&(x.clone() + vx.clone() * t.clone())._eq(&(a + va * t.clone())));
        solver.assert(&(y.clone() + vy.clone() * t.clone())._eq(&(b + vb * t.clone())));
        solver.assert(&(z.clone() + vz.clone() * t.clone())._eq(&(c + vc * t.clone())));
    }

    println!("Running...");
    if solver.check() == z3::SatResult::Sat {
        let Some(m) = solver.get_model() else {
            panic!("failed to solve model")
        };
        return m.eval(&(x + y + z), true).unwrap().as_u64();
    }
    panic!("failed to solve model")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_solution_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(17244));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(47));
    }
}
