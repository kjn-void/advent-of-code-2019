use super::intcode::*;
use super::Solution;
use std::collections::HashMap;
use std::sync::mpsc::*;

type Coord = (i32, i32);

#[derive(PartialEq)]
enum Turn {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Paint {
    Black,
    White,
}

// State required to solve day 11
pub struct State {
    program: Vec<Intcode>,
}

fn paint_hull(program: &Vec<Intcode>, start_tile_col: Paint) -> HashMap<Coord, Paint> {
    let (input, sink) = channel();
    let output = exec(program, sink, None);
    let mut hull = HashMap::new();
    let mut pos = (0, 0);
    let mut dir = (0, 1); // Facing up
    let mut color_cur_tile = start_tile_col;
    loop {
        if let Err(_) = input.send(if color_cur_tile == Paint::Black { 0 } else { 1 }) {
            break;
        };
        let color = if let Ok(col) = output.recv() {
            if col == 0 {
                Paint::Black
            } else {
                Paint::White
            }
        } else {
            break;
        };
        dir = match if output.recv().unwrap() == 0 {
            Turn::Left
        } else {
            Turn::Right
        } {
            Turn::Left => (-dir.1, dir.0),
            Turn::Right => (dir.1, -dir.0),
        };
        hull.insert(pos, color);
        pos.0 = pos.0 + dir.0;
        pos.1 = pos.1 + dir.1;
        color_cur_tile = if let Some(&color) = hull.get(&pos) {
            color
        } else {
            Paint::Black
        };
    }
    hull
}

fn bound_box(hull: &HashMap<Coord, Paint>) -> (Coord, Coord) {
    let mut top_left = *hull.keys().next().unwrap();
    let mut bottom_right = top_left;
    for &pos in hull.keys() {
        if top_left.0 > pos.0 {
            top_left.0 = pos.0;
        }
        if top_left.1 > pos.1 {
            top_left.1 = pos.1;
        }
        if bottom_right.0 < pos.0 {
            bottom_right.0 = pos.0;
        }
        if bottom_right.1 < pos.1 {
            bottom_right.1 = pos.1;
        }
    }
    (top_left, bottom_right)
}

impl Solution for State {
    fn part1(&self) -> String {
        paint_hull(&self.program, Paint::Black).len().to_string()
    }

    fn part2(&self) -> String {
        let hull = paint_hull(&self.program, Paint::White);
        let (tl, br) = bound_box(&hull);
        let mut plate = Vec::new();
        for y in (tl.1..=br.1).rev() {
            plate.push('\n');
            for x in tl.0..=br.0 {
                let pos = (x as i32, y as i32);
                let ch = if let Some(&col) = hull.get(&pos) {
                    if col == Paint::Black { '▒' } else { '█' }
                } else {
                    '▒'
                };
                plate.push(ch);
            }
        };
        plate.iter().collect()
    }
}

pub fn solution(lines: Vec<&str>) -> Box<dyn Solution> {
    Box::new(State {
        program: lines[0]
            .split(",")
            .map(|ic| ic.parse::<Intcode>().unwrap())
            .collect(),
    })
}
