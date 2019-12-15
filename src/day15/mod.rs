use super::intcode::*;
use super::Solution;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::mpsc::*;

type Map = HashMap<Vec2D, Tile>;

#[derive(Debug, PartialEq)]
enum Dir {
    North,
    South,
    West,
    East,
}

// Tuple with direction forward and direction to undo
fn to_dirs(to: Vec2D, from: Vec2D) -> (Dir, Dir) {
    if to.x == from.x {
        if to.y > from.y {
            (Dir::North, Dir::South)
        } else {
            (Dir::South, Dir::North)
        }
    } else {
        if to.x > from.x {
            (Dir::East, Dir::West)
        } else {
            (Dir::West, Dir::East)
        }
    }
}

fn from_dir(dir: Dir) -> Intcode {
    match dir {
        Dir::North => 1,
        Dir::South => 2,
        Dir::West => 3,
        _ => 4,
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Tile {
    Wall,
    Space,
    Oxygen,
}

fn to_tile(ic: Intcode) -> Tile {
    match ic {
        0 => Tile::Wall,
        1 => Tile::Space,
        2 => Tile::Oxygen,
        _ => panic!("Invalid feedback code"),
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Vec2D {
    x: i32,
    y: i32,
}

fn next_pos(pos: Vec2D, pred: &dyn Fn(&Vec2D) -> bool) -> Vec<Vec2D> {
    vec![
        Vec2D { x: pos.x, y: pos.y + 1 },
        Vec2D { x: pos.x, y: pos.y - 1 },
        Vec2D { x: pos.x - 1, y: pos.y },
        Vec2D { x: pos.x + 1, y: pos.y },
    ]
    .into_iter()
    .filter(pred)
    .collect()
}

fn depth_first(
    pos: Vec2D,
    map: &mut Map,
    joystick: &Sender<Intcode>,
    droid: &Receiver<Intcode>,
) {
    for np in next_pos(pos, &|p| !map.contains_key(p)) {
        let (dir, undo) = to_dirs(np, pos);
        joystick.send(from_dir(dir)).unwrap();
        let tile = to_tile(droid.recv().unwrap());
        map.insert(np, tile);
        if tile != Tile::Wall {
            depth_first(np, map, joystick, droid);
            joystick.send(from_dir(undo)).unwrap();
            if to_tile(droid.recv().unwrap()) != Tile::Space {
                panic!("Failed to undo move");
            }
        }
    }
}

fn explore_map(program: &Vec<Intcode>) -> Map {
    let (joystick, sink) = channel();
    let output = exec(program, sink, None);
    let mut map = HashMap::new();
    map.insert(Vec2D { x: 0, y: 0 }, Tile::Space);
    depth_first(Vec2D { x: 0, y: 0 }, &mut map, &joystick, &output);
    map
}

fn fill_map_with_oxygen(map: &mut Map) -> (u32, u32) {
    let start_pos = Vec2D { x: 0, y: 0 };
    let &oxygen_pos = map.iter().filter(|(_, &v)| v == Tile::Oxygen).next().unwrap().0;
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
