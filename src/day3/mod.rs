use regex::Regex;
use std::collections::HashMap;
use super::Solution;
use super::vec2d::*;

struct Movement {
    dir: Dir,
    distance: Distance,
}

fn dir_parse(repr: &str) -> Dir {
    match repr {
        "U" => Dir::Up,
        "D" => Dir::Down,
        "R" => Dir::Right,
        "L" => Dir::Left,
        _ => panic!("Invalid direction"),
    }
}

// Returns all position a line passes through and the distance from start for
// each position.
fn line_trace(line: &Vec<Movement>) -> HashMap<Vec2D, Distance> {
    let mut trace = HashMap::new();
    let mut p = Vec2D::default();
    let mut distance = 0;
    for m in line {
        let path = p.step_n(m.distance, m.dir);
        for pos in &path {
            distance = distance + 1;
            trace.insert(*pos, distance);
        }
    }
    trace
}

impl Solution for State {
    fn part1(&self) -> String {
        let trace = line_trace(&self.line_a);
        let mut p = Vec2D::default();
        self.line_b
            .iter()
            .flat_map(|m| p.step_n(m.distance, m.dir))
            .filter(|pos| trace.contains_key(pos))
            .map(|pos| pos.manhattan_distance(Vec2D::default()))
            .min()
            .unwrap()
            .to_string()
    }

    fn part2(&self) -> String {
        let trace = line_trace(&self.line_a);
        let mut min_distance = std::u32::MAX;
        let mut dist_b = 0;
        let mut p = Vec2D::default();
        for m in &self.line_b {
            let path = p.step_n(m.distance, m.dir);
            for pos in &path {
                dist_b = dist_b + 1;
                if let Some(dist_a) = trace.get(pos) {
                    min_distance = std::cmp::min(min_distance, dist_a + dist_b);
                }
            }
        }
        min_distance.to_string()
    }
}

// State required to solve day 3
pub struct State {
    line_a: Vec<Movement>,
    line_b: Vec<Movement>,
}

pub fn solution(lines: Vec<&str>) -> Box<dyn Solution> {
    let mut sln = State {
        line_a: Vec::new(),
        line_b: Vec::new(),
    };
    let re = Regex::new(r"([UDLR])(\d+)(,|$)").unwrap();
    for (idx, line) in lines.iter().enumerate() {
        for grp in re.captures_iter(line) {
            let mov = Movement {
                dir: dir_parse(&grp[1]),
                distance: grp[2].parse::<Distance>().unwrap(),
            };
            if idx == 0 {
                sln.line_a.push(mov);
            } else {
                sln.line_b.push(mov);
            }
        }
    }
    Box::new(sln)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d3_ex1() {
        let input = vec![
            "R75,D30,R83,U83,L12,D49,R71,U7,L72",
            "U62,R66,U55,R34,D71,R55,D58,R83",
        ];
        assert!(solution(input).part1() == "159");
    }
    #[test]
    fn d3_ex2() {
        let input = vec![
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        ];
        assert!(solution(input).part1() == "135");
    }
    #[test]
    fn d3_ex3() {
        let input = vec![
            "R75,D30,R83,U83,L12,D49,R71,U7,L72",
            "U62,R66,U55,R34,D71,R55,D58,R83",
        ];
        assert!(solution(input).part2() == "610");
    }
    #[test]
    fn d3_ex4() {
        let input = vec![
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        ];
        assert!(solution(input).part2() == "410");
    }
}
