use super::intcode::*;
use super::vec2d::*;
use super::Solution;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::mpsc::*;

type Map = HashMap<Vec2D, Tile>;

enum Action {
    Do((Compass, Vec2D)),
    Undo((Compass, Vec2D)),
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Tile {
    Wall,
    Space,
    Oxygen,
}

fn intcode_to_tile(ic: Intcode) -> Tile {
    match ic {
        0 => Tile::Wall,
        1 => Tile::Space,
        2 => Tile::Oxygen,
        _ => panic!("Invalid feedback code"),
    }
}

fn compass_to_intcode(compass: Compass) -> Intcode {
    match compass {
        Compass::North => 1,
        Compass::South => 2,
        Compass::West => 3,
        _ => 4,
    }
}

fn next_pos(pos: Vec2D, pred: &dyn Fn(&Vec2D) -> bool) -> Vec<Vec2D> {
    vec![NORTH, SOUTH, WEST, EAST]
        .iter()
        .map(|&d| pos + d)
        .filter(pred)
        .collect()
}

fn explore_map(program: &Vec<Intcode>) -> Map {
    let (joystick, sink) = channel();
    let droid = exec(program, sink, None);
    let mut map = HashMap::new();
    let mut pos = Vec2D::default();
    let mut stack = Vec::new();
    loop {
        for next_pos in next_pos(pos, &|p| !map.contains_key(p)) {
            let dir = pos.compass(next_pos);
            let undo = next_pos.compass(pos);
            stack.push(Action::Undo((undo, pos)));
            stack.push(Action::Do((dir, next_pos)));
        }
        if let Some(action) = stack.pop() {
            match action {
                Action::Do((compass, next_pos)) => {
                    joystick.send(compass_to_intcode(compass)).unwrap();
                    let tile = intcode_to_tile(droid.recv().unwrap());
                    map.insert(next_pos, tile);
                    if tile == Tile::Wall {
                        stack.pop();
                    } else {
                        pos = next_pos;
                    }
                }
                Action::Undo((compass, prev_pos)) => {
                    joystick.send(compass_to_intcode(compass)).unwrap();
                    if intcode_to_tile(droid.recv().unwrap()) != Tile::Space {
                        panic!("Failed to undo move");
                    }
                    pos = prev_pos;
                }
            }
        } else {
            break map;
        }
    }
}

fn fill_map_with_oxygen(map: &mut Map) -> (u32, u32) {
    let start_pos = Vec2D::default();
    let &oxygen_pos = map
        .iter()
        .filter(|(_, &v)| v == Tile::Oxygen)
        .next()
        .unwrap()
        .0;
    let mut q = VecDeque::new();
    q.push_back((oxygen_pos, 0));
    let mut steps_to_start = 0;
    let mut steps_to_fill = 0;
    // Breadth-first search with oxygen
    while let Some((pos, steps)) = q.pop_front() {
        for np in next_pos(pos, &|p| map.get(p).unwrap() == &Tile::Space) {
            q.push_back((np, steps + 1));
            map.insert(np, Tile::Oxygen);
        }
        if pos == start_pos {
            steps_to_start = steps;
        }
        steps_to_fill = steps;
    }
    (steps_to_start, steps_to_fill)
}

impl Solution for State {
    fn part1(&self) -> String {
        let mut map = explore_map(&self.program);
        let (steps_to_oxygen, _) = fill_map_with_oxygen(&mut map);
        steps_to_oxygen.to_string()
    }

    fn part2(&self) -> String {
        let mut map = explore_map(&self.program);
        let (_, min_to_fill) = fill_map_with_oxygen(&mut map);
        min_to_fill.to_string()
    }
}

// State required to solve day 15
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
