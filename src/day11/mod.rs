use super::intcode::*;
use super::vec2d::*;
use super::Solution;
use std::collections::HashMap;
use std::sync::mpsc::*;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Paint {
    Black,
    White,
}

fn paint_hull(program: &Vec<Intcode>, start_tile_col: Paint) -> HashMap<Vec2D, Paint> {
    let (input, sink) = channel();
    let output = exec(program, sink, None);
    let mut hull = HashMap::new();
    let mut pos = Vec2D::default();
    let mut dir = Vec2D::from_dir(Dir::Up);
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
        dir = match output.recv().unwrap() {
            0 => dir.turn(Turn::Counterclockwise),
            _ => dir.turn(Turn::Clockwise),
        };
        hull.insert(pos, color);
        pos += dir;
        color_cur_tile = if let Some(&color) = hull.get(&pos) {
            color
        } else {
            Paint::Black
        };
    }
    hull
}

fn bound_box(hull: &HashMap<Vec2D, Paint>) -> (Vec2D, Vec2D) {
    let top = hull.keys().map(|p| p.y()).min().unwrap();
    let left = hull.keys().map(|p| p.x()).min().unwrap();
    let bottom = hull.keys().map(|p| p.y()).max().unwrap();
    let right = hull.keys().map(|p| p.x()).max().unwrap();
    (Vec2D::from(left, top), Vec2D::from(right, bottom))
}

impl Solution for State {
    fn part1(&self) -> String {
        paint_hull(&self.program, Paint::Black).len().to_string()
    }

    fn part2(&self) -> String {
        let hull = paint_hull(&self.program, Paint::White);
        let (tl, br) = bound_box(&hull);
        let mut plate = Vec::new();
        for y in (tl.y()..=br.y()).rev() {
            plate.push('\n');
            for x in tl.x()..=br.x() {
                let pos = Vec2D::from(x, y);
                let ch = if let Some(&col) = hull.get(&pos) {
                    if col == Paint::Black {
                        '▒'
                    } else {
                        '█'
                    }
                } else {
                    '▒'
                };
                plate.push(ch);
            }
        }
        plate.iter().collect()
    }
}

// State required to solve day 11
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
