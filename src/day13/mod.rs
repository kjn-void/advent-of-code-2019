use super::intcode::*;
use super::Solution;
use std::collections::HashMap;
use std::sync::mpsc::*;

type Screen = HashMap<Vec2D, Tile>;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Vec2D {
    x: i32,
    y: i32,
}

impl Default for Vec2D {
    fn default() -> Self {
        Vec2D { x: 0, y: 0 }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
    Score(i32),
}

// State required to solve day 13
pub struct State {
    program: Vec<Intcode>,
}

fn to_tile(tile_id: Intcode, x: Intcode, y: Intcode) -> Tile {
    if x == -1 && y == 0 {
        return Tile::Score(tile_id as i32);
    }
    match tile_id {
        0 => Tile::Empty,
        1 => Tile::Wall,
        2 => Tile::Block,
        3 => Tile::Paddle,
        4 => Tile::Ball,
        _ => panic!("Invalid tile ID"),
    }
}

fn next_tile(output: &Receiver<Intcode>) -> Option<(Vec2D, Tile)> {
    if let Ok(x) = output.recv() {
        let y = output.recv().unwrap();
        let tile_id = output.recv().unwrap();
        Some((
            Vec2D {
                x: x as i32,
                y: y as i32,
            },
            to_tile(tile_id, x, y),
        ))
    } else {
        None
    }
}

fn render(screen: &Screen) {
    let mut line = 0;
    let mut x = 0;
    let max_y = screen.keys().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y;
    print!("{}[2J", 27 as char); // clear console
    if let Some(Tile::Score(val)) = screen.get(&Vec2D { x: -1, y: 0 }) {
        println!("Score: {}", val);
    }
    loop {
        let pos = Vec2D { x: x, y: line };
        x = x + 1;
        if let Some(tile) = screen.get(&pos) {
            print!(
                "{}",
                match tile {
                    Tile::Ball => "⚾",
                    Tile::Block => "▒▒",
                    Tile::Empty => "  ",
                    Tile::Paddle => "▃▃",
                    _ => "██",
                }
            )
        } else {
            x = 0;
            println!("");
            line = line + 1;
            if line == max_y + 1 {
                break;
            }
        }
    }
    std::thread::sleep(std::time::Duration::from_millis(50));
}

impl Solution for State {
    fn part1(&self) -> String {
        let (_, sink) = channel();
        let output = exec(&self.program, sink, None);
        let mut blocks = 0;
        while let Some(tile) = next_tile(&output) {
            if tile.1 == Tile::Block {
                blocks = blocks + 1;
            }
        }
        blocks.to_string()
    }

    fn part2(&self) -> String {
        let mut free_play_program = self.program.clone();
        free_play_program[0] = 2;
        let (joystick, sink) = channel();
        let output = exec(&free_play_program, sink, None);
        let mut paddle = Vec2D::default();
        let mut final_score = 0;
        let mut screen = Screen::new();
        while let Some(tile) = next_tile(&output) {
            screen.insert(tile.0, tile.1);
            match tile.1 {
                Tile::Score(val) => final_score = val,
                Tile::Paddle => {
                    paddle = tile.0;
                    render(&screen);
                }
                Tile::Ball => {
                    let ball = tile.0;
                    joystick.send(
                        if ball.x < paddle.x {
                            -1
                        } else if ball.x > paddle.x {
                            1
                        } else {
                            0
                        })
                        .unwrap();
                }
                _ => (),
            }
        }
        final_score.to_string()
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
