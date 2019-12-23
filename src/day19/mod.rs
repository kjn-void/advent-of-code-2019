use std::sync::mpsc::*;
use super::intcode::*;
use super::Solution;
use super::vec2d::*;

fn is_in_beam(pos: Vec2D, program: &Vec<Intcode>) -> bool {
    let (input, sink) = channel();
    let output = exec(program, sink, None);
    input.send(pos.x() as Intcode).unwrap();
    input.send(pos.y() as Intcode).unwrap();
    output.recv().unwrap() != 0
}

impl Solution for State {
    fn part1(&self) -> String {
        (0..50)
            .flat_map(|y| {
                (0..50).map(|x| {
                    is_in_beam(Vec2D::from(x, y), &self.program)
                }).collect::<Vec<_>>()
            })
            .filter(|&in_beam| in_beam)
            .count()
            .to_string()
    }

    fn part2(&self) -> String {
        let is_in_beam = |pos| is_in_beam(pos, &self.program);
        let top = Vec2D::from(0, -99);
        let top_right = Vec2D::from(99, -99);
        let mut pos = Vec2D::from(0, 100);
        loop {
            while !is_in_beam(pos) {
                pos += RIGHT;
            }
            let top_pos = pos + top;
            if is_in_beam(top_pos) && is_in_beam(pos + top_right) {
                break top_pos.x() * 10000 + top_pos.y();
            }
            pos += DOWN;
        }.to_string()
    }
}

// State required to solve day 19
pub struct State {
    program: Vec<Intcode>,
}

pub fn solution(lines: Vec<&str>) -> Box<dyn Solution> {
    Box::new(State {
        program: lines[0]
            .split(",")
            .map(|ic| ic.parse::<Intcode>().unwrap())
            .collect(),
    })
}
