use super::intcode::*;
use super::vec2d::*;
use super::Solution;
use regex::Regex;
use std::char;
use std::collections::HashMap;
use std::env;
use std::sync::mpsc::channel;

type Map = HashMap<Vec2D, Tile>;
type Movement = (Turn, Distance);

#[derive(Debug, PartialEq)]
enum Tile {
    Scaffold,
    Space,
    Robot(Dir),
}

fn map_get(program: &Vec<Intcode>) -> Map {
    let (_, sink) = channel();
    let camera = exec(program, sink, None);
    let mut map = Map::new();
    let mut pos = Vec2D::default();
    while let Ok(feed) = camera.recv() {
        let camera_feedback = char::from_u32(feed as u32).unwrap();
        match camera_feedback {
            '#' => map.insert(pos, Tile::Scaffold),
            '<' => map.insert(pos, Tile::Robot(Dir::Left)),
            '>' => map.insert(pos, Tile::Robot(Dir::Right)),
            '^' => map.insert(pos, Tile::Robot(Dir::Up)),
            'v' => map.insert(pos, Tile::Robot(Dir::Down)),
            '.' => map.insert(pos, Tile::Space),
            '\n' => None,
            _ => panic!("Invalid camera feedback"),
        };
        pos = if camera_feedback == '\n' {
            Vec2D::from(0, pos.y() + 1)
        } else {
            pos + RIGHT
        }
    }
    map
}

fn is_intersection(map: &Map, pos: Vec2D) -> bool {
    vec![Vec2D::default(), UP, DOWN, LEFT, RIGHT]
        .into_iter()
        .fold(true, |pred, offset| {
            pred && map.get(&(pos + offset)).unwrap_or(&Tile::Space) != &Tile::Space
        })
}

fn next_turn(pos: Vec2D, dir: Dir, map: &Map) -> Option<Turn> {
    let turns = [Turn::Left, Turn::Right];
    if let Some(&turn) = turns
        .iter()
        .filter(|&&to| {
            let new_pos = pos + Vec2D::from_dir(dir.turn(to));
            map.get(&new_pos).unwrap_or(&Tile::Space) == &Tile::Scaffold
        })
        .last()
    {
        return Some(turn);
    }
    None
}

fn path_get(map: &Map) -> Vec<Movement> {
    let mut moves = Vec::new();
    let (start_pos, start_dir) = map
        .iter()
        .filter(|(_, tile)| {
            if let Tile::Robot(_) = tile {
                true
            } else {
                false
            }
        })
        .next()
        .unwrap();
    let mut pos = *start_pos;
    let mut dir = if let Tile::Robot(d) = start_dir {
        *d
    } else {
        panic!("BUG!")
    };
    while let Some(to) = next_turn(pos, dir, map) {
        dir = dir.turn(to);
        let mut steps = 0;
        loop {
            let new_pos = pos + Vec2D::from_dir(dir);
            if map.get(&new_pos).unwrap_or(&Tile::Space) != &Tile::Scaffold {
                break;
            }
            pos = new_pos;
            steps += 1;
        }
        moves.push((to, steps));
    }
    moves
}

fn path_to_string(path: Vec<Movement>) -> String {
    path.into_iter().fold("".to_string(), |acc, (turn, steps)| {
        acc + turn.to_str() + "," + &steps.to_string() + ","
    })
}

fn remove_n(path: &String, n: usize) -> (String, String) {
    let sub_prog = path.split(",").take(n * 2).collect::<Vec<_>>().join(",");
    let re = Regex::new(&(sub_prog.clone() + ",")).unwrap();
    (sub_prog.clone(), re.replace_all(path, "").to_string())
}

fn compile_with(path: &String, a: &String, b: &String, c: &String) -> Option<String> {
    let mut prog = "".to_string();
    let mut p = path.clone();
    let re = [a, b, c]
        .iter()
        .map(|&sub_prog| Regex::new(&format!("^{},", sub_prog)).unwrap())
        .collect::<Vec<_>>();
    while p.len() > 0 {
        if let Some((re, &sub_prog)) = re
            .iter()
            .zip(&["A", "B", "C"])
            .filter(|(re, _)| re.find(&p) != None)
            .next()
        {
            p = re.replace(&p, "").to_string();
            prog += sub_prog;
            prog += ",";
        } else {
            return None;
        }
    }
    Some(prog)
}

fn compile(path: String) -> (String, String, String, String) {
    for a in 1..=10 {
        let (a_prog, rem_path) = remove_n(&path, a);
        for b in 1..=10 {
            let (b_prog, rem_path) = remove_n(&rem_path, b);
            for c in 1..=10 {
                let (c_prog, rem_path) = remove_n(&rem_path, c);
                if rem_path.len() == 0 {
                    if let Some(mut main_rtn) = compile_with(&path, &a_prog, &b_prog, &c_prog) {
                        main_rtn.pop();
                        main_rtn.push('\n');
                        if main_rtn.len() <= 20 {
                            return (main_rtn, a_prog + "\n", b_prog + "\n", c_prog + "\n");
                        }
                    }
                }
            }
        }
    }
    panic!("No solution found");
}

impl Solution for Day17 {
    fn part1(&self) -> String {
        let map = map_get(&self.program);
        if self.verbose {
            render(&map);
        }
        map.keys()
            .filter(|&&pos| is_intersection(&map, pos))
            .map(|pos| pos.x() * pos.y())
            .sum::<Coord>()
            .to_string()
    }

    fn part2(&self) -> String {
        let map = map_get(&self.program);
        let (input, sink) = channel();
        let mut program = self.program.clone();
        program[0] = 2;
        let output = exec(&program, sink, None);

        let path = path_to_string(path_get(&map));
        let (main_rtn, a, b, c) = compile(path);
        for ch in main_rtn.chars().chain(
            a.chars()
                .chain(b.chars().chain(c.chars().chain("n\n".chars()))),
        ) {
            if self.verbose {
                print!("{}", ch);
            }
            input.send(ch as i64).unwrap();
        }
        let mut dust = 0;
        while let Ok(ch) = output.recv() {
            dust = ch;
        }
        dust.to_string()
    }
}

// State required to solve day 17
pub struct Day17 {
    program: Vec<Intcode>,
    verbose: bool,
}

pub fn solution(lines: Vec<&str>) -> Box<dyn Solution> {
    Box::new(Day17 {
        program: lines[0]
            .split(",")
            .map(|ic| ic.parse::<Intcode>().unwrap())
            .collect(),
        verbose: env::args().last().unwrap() == "-v",
    })
}

fn render(map: &Map) {
    let mut pos = Vec2D::default();
    loop {
        let tile = map.get(&pos);
        if tile == None {
            println!();
            if pos.x() == 0 {
                break;
            }
            pos = Vec2D::from(0, pos.y() + 1);
        } else if let Some(tile) = tile {
            print!(
                "{}",
                match tile {
                    Tile::Space => " ",
                    Tile::Scaffold => {
                        if is_intersection(&map, pos) {
                            "O"
                        } else {
                            "#"
                        }
                    }
                    Tile::Robot(d) => d.to_str(),
                }
            );
            pos += RIGHT;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "1,330,331,332,109,2734,1102,1182,1,15,1102,1,1429,24,1002,0,1,570,1006,570,36,1001,571,0,0,1001,570,-1,570,1001,24,1,24,1106,0,18,1008,571,0,571,1001,15,1,15,1008,15,1429,570,1006,570,14,21102,58,1,0,1105,1,786,1006,332,62,99,21101,0,333,1,21102,73,1,0,1105,1,579,1101,0,0,572,1101,0,0,573,3,574,101,1,573,573,1007,574,65,570,1005,570,151,107,67,574,570,1005,570,151,1001,574,-64,574,1002,574,-1,574,1001,572,1,572,1007,572,11,570,1006,570,165,101,1182,572,127,1001,574,0,0,3,574,101,1,573,573,1008,574,10,570,1005,570,189,1008,574,44,570,1006,570,158,1105,1,81,21102,1,340,1,1105,1,177,21101,0,477,1,1106,0,177,21101,0,514,1,21101,0,176,0,1106,0,579,99,21102,1,184,0,1106,0,579,4,574,104,10,99,1007,573,22,570,1006,570,165,1001,572,0,1182,21102,1,375,1,21101,211,0,0,1106,0,579,21101,1182,11,1,21102,1,222,0,1105,1,979,21102,1,388,1,21102,233,1,0,1105,1,579,21101,1182,22,1,21101,0,244,0,1106,0,979,21101,401,0,1,21101,0,255,0,1105,1,579,21101,1182,33,1,21102,266,1,0,1106,0,979,21102,1,414,1,21102,1,277,0,1106,0,579,3,575,1008,575,89,570,1008,575,121,575,1,575,570,575,3,574,1008,574,10,570,1006,570,291,104,10,21102,1182,1,1,21101,313,0,0,1106,0,622,1005,575,327,1102,1,1,575,21102,327,1,0,1105,1,786,4,438,99,0,1,1,6,77,97,105,110,58,10,33,10,69,120,112,101,99,116,101,100,32,102,117,110,99,116,105,111,110,32,110,97,109,101,32,98,117,116,32,103,111,116,58,32,0,12,70,117,110,99,116,105,111,110,32,65,58,10,12,70,117,110,99,116,105,111,110,32,66,58,10,12,70,117,110,99,116,105,111,110,32,67,58,10,23,67,111,110,116,105,110,117,111,117,115,32,118,105,100,101,111,32,102,101,101,100,63,10,0,37,10,69,120,112,101,99,116,101,100,32,82,44,32,76,44,32,111,114,32,100,105,115,116,97,110,99,101,32,98,117,116,32,103,111,116,58,32,36,10,69,120,112,101,99,116,101,100,32,99,111,109,109,97,32,111,114,32,110,101,119,108,105,110,101,32,98,117,116,32,103,111,116,58,32,43,10,68,101,102,105,110,105,116,105,111,110,115,32,109,97,121,32,98,101,32,97,116,32,109,111,115,116,32,50,48,32,99,104,97,114,97,99,116,101,114,115,33,10,94,62,118,60,0,1,0,-1,-1,0,1,0,0,0,0,0,0,1,6,0,0,109,4,1202,-3,1,587,20101,0,0,-1,22101,1,-3,-3,21101,0,0,-2,2208,-2,-1,570,1005,570,617,2201,-3,-2,609,4,0,21201,-2,1,-2,1105,1,597,109,-4,2106,0,0,109,5,2102,1,-4,630,20102,1,0,-2,22101,1,-4,-4,21102,0,1,-3,2208,-3,-2,570,1005,570,781,2201,-4,-3,652,21002,0,1,-1,1208,-1,-4,570,1005,570,709,1208,-1,-5,570,1005,570,734,1207,-1,0,570,1005,570,759,1206,-1,774,1001,578,562,684,1,0,576,576,1001,578,566,692,1,0,577,577,21101,0,702,0,1105,1,786,21201,-1,-1,-1,1106,0,676,1001,578,1,578,1008,578,4,570,1006,570,724,1001,578,-4,578,21101,0,731,0,1106,0,786,1105,1,774,1001,578,-1,578,1008,578,-1,570,1006,570,749,1001,578,4,578,21101,0,756,0,1106,0,786,1106,0,774,21202,-1,-11,1,22101,1182,1,1,21102,774,1,0,1105,1,622,21201,-3,1,-3,1105,1,640,109,-5,2106,0,0,109,7,1005,575,802,21002,576,1,-6,20101,0,577,-5,1105,1,814,21102,1,0,-1,21102,1,0,-5,21102,0,1,-6,20208,-6,576,-2,208,-5,577,570,22002,570,-2,-2,21202,-5,29,-3,22201,-6,-3,-3,22101,1429,-3,-3,1202,-3,1,843,1005,0,863,21202,-2,42,-4,22101,46,-4,-4,1206,-2,924,21102,1,1,-1,1105,1,924,1205,-2,873,21102,1,35,-4,1105,1,924,1202,-3,1,878,1008,0,1,570,1006,570,916,1001,374,1,374,2101,0,-3,895,1101,2,0,0,2102,1,-3,902,1001,438,0,438,2202,-6,-5,570,1,570,374,570,1,570,438,438,1001,578,558,922,20101,0,0,-4,1006,575,959,204,-4,22101,1,-6,-6,1208,-6,29,570,1006,570,814,104,10,22101,1,-5,-5,1208,-5,45,570,1006,570,810,104,10,1206,-1,974,99,1206,-1,974,1101,1,0,575,21101,973,0,0,1105,1,786,99,109,-7,2105,1,0,109,6,21101,0,0,-4,21102,1,0,-3,203,-2,22101,1,-3,-3,21208,-2,82,-1,1205,-1,1030,21208,-2,76,-1,1205,-1,1037,21207,-2,48,-1,1205,-1,1124,22107,57,-2,-1,1205,-1,1124,21201,-2,-48,-2,1105,1,1041,21102,-4,1,-2,1106,0,1041,21101,-5,0,-2,21201,-4,1,-4,21207,-4,11,-1,1206,-1,1138,2201,-5,-4,1059,1201,-2,0,0,203,-2,22101,1,-3,-3,21207,-2,48,-1,1205,-1,1107,22107,57,-2,-1,1205,-1,1107,21201,-2,-48,-2,2201,-5,-4,1090,20102,10,0,-1,22201,-2,-1,-2,2201,-5,-4,1103,2101,0,-2,0,1106,0,1060,21208,-2,10,-1,1205,-1,1162,21208,-2,44,-1,1206,-1,1131,1105,1,989,21101,439,0,1,1105,1,1150,21102,1,477,1,1105,1,1150,21102,1,514,1,21102,1,1149,0,1105,1,579,99,21101,1157,0,0,1106,0,579,204,-2,104,10,99,21207,-3,22,-1,1206,-1,1138,2101,0,-5,1176,1202,-4,1,0,109,-6,2105,1,0,6,5,28,1,28,1,28,1,28,1,28,1,20,11,1,7,10,1,7,1,1,1,1,1,5,1,10,1,5,11,1,1,10,1,5,1,1,1,1,1,1,1,3,1,1,1,10,9,1,1,1,1,1,5,16,1,3,1,1,1,1,1,1,1,18,5,1,1,1,1,1,1,24,1,1,1,1,1,24,1,1,1,1,1,24,1,1,1,1,1,24,5,7,1,18,1,9,1,18,1,9,1,18,1,9,1,18,1,9,1,18,1,9,1,12,7,9,1,12,1,15,1,12,1,15,1,12,1,15,1,12,1,15,1,12,1,15,1,2,5,5,1,9,7,2,1,3,1,5,1,9,1,8,1,3,1,5,1,9,1,8,1,3,1,5,1,9,1,8,1,3,13,3,1,8,1,9,1,5,1,3,1,8,11,5,1,1,5,22,1,1,1,1,1,1,1,22,1,1,1,1,1,1,1,22,1,1,1,1,1,1,1,22,13,18,1,1,1,1,1,5,1,16,5,1,1,5,1,16,1,1,1,3,1,5,1,16,1,1,11,16,1,5,1,22,7,6";

    #[test]
    fn d17_part1() {
        assert_eq!(solution(vec![INPUT]).part1(), "5788");
    }

    #[test]
    fn d17_part2() {
        assert_eq!(solution(vec![INPUT]).part2(), "648545");
    }
}
