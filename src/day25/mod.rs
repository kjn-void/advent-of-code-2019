use super::intcode::*;
use super::vec2d::Compass;
use super::Solution;
use std::collections::HashSet;
use std::env;
use std::iter::FromIterator;
use std::sync::mpsc::*;

type Room = String;
type Map = HashSet<(Room, Compass)>;
type Exits = Vec<Compass>;
type Item = String;
type Items = Vec<Item>;
type ItemBag = u32;

struct Robot {
    carrying: ItemBag,
    seen: Items,
    last_move: Option<Compass>,
    checkpoint_found: bool,
    path_to_checkpoint: Vec<Compass>,
    input: Sender<Intcode>,
    output: Receiver<Intcode>,
    verbose: bool,
}

fn room_info_lines(output: &Receiver<Intcode>, verbose: bool) -> Vec<String> {
    let mut desc_buf = Vec::new();
    while let Ok(ch) = output.recv() {
        desc_buf.push((ch as u8) as char);
        if desc_buf.iter().collect::<String>().ends_with("Command?")
        {
            break;
        }
    }
    let desc_str = desc_buf.iter().collect::<String>();
    if verbose {
        println!("{}", desc_str);
    }
    desc_str
        .trim()
        .split('\n')
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
}

fn name_get(lines: &Vec<String>) -> Room {
    lines[0]
        .trim_matches(|ch: char| !ch.is_ascii_alphabetic())
        .to_string()
}

fn exits_get(lines: &Vec<String>) -> Exits {
    let mut exits = Exits::new();
    let mut in_exits = false;
    for line in lines {
        if in_exits {
            if line.len() == 0 {
                in_exits = false;
            } else {
                exits.push(match line as &str {
                    "- north" => Compass::North,
                    "- east" => Compass::East,
                    "- south" => Compass::South,
                    "- west" => Compass::West,
                    _ => panic!("Invalid direction"),
                });
            }
        } else if line == "Doors here lead:" {
            in_exits = true;
        }
    }
    exits
}

fn item_get(lines: &Vec<String>) -> Option<Item> {
    let mut item = None;
    let mut in_items = false;
    for line in lines {
        if in_items {
            if line.len() == 0 {
                in_items = false;
            } else {
                item = Some(line.chars().skip(2).collect());
            }
        } else if line == "Items here:" {
            in_items = true;
        }
    }
    item
}

fn room_info(output: &Receiver<Intcode>, verbose: bool) -> (Room, Exits, Option<Item>) {
    let lines = room_info_lines(output, verbose);
    (name_get(&lines), exits_get(&lines), item_get(&lines))
}

fn issue_cmd(cmd: String, input: &Sender<Intcode>, verbose: bool) {
    for ch in cmd.chars() {
        input.send(ch as Intcode).unwrap();
        if verbose {
            print!("{}", ch);
        }
    }
}

impl Robot {
    fn item_pick(&mut self, item: &Item) {
        issue_cmd("take ".to_string() + item + "\n", &self.input, self.verbose);
        self.carrying |= 1 << self.seen.len();
        self.seen.push(item.clone());
    }

    fn item_drop(&mut self, item: &Item) {
        issue_cmd("drop ".to_string() + item + "\n", &self.input, self.verbose);
    }

    fn move_to(&mut self, compass: Compass, new_area: bool) {
        issue_cmd(format!("{:?}\n", compass).to_lowercase(), &self.input, self.verbose);
        self.last_move = Some(compass);
        if !self.checkpoint_found {
            if new_area {
                self.path_to_checkpoint.push(compass);
            } else {
                self.path_to_checkpoint.pop().unwrap();
            }
        }
    }
}

fn gather_items(robot: &mut Robot) {
    let blacklist: HashSet<Item> = HashSet::from_iter(
        [
            "giant electromagnet",
            "escape pod",
            "photons",
            "molten lava",
            "infinite loop",
        ]
        .iter()
        .map(|s| s.to_string()),
    );
    let mut visited = Map::new();
    let mut stack = Vec::new();
    loop {
        let (name, exits, item) = room_info(&robot.output, robot.verbose);
        if exits.len() > 0 {
            if let Some(last_move) = robot.last_move {
                visited.insert((name.clone(), last_move.mirror()));
            }
        }
        if name == "Security Checkpoint" {
            robot.checkpoint_found = true;
        } else {
            for &exit in &exits {
                if !visited.contains(&(name.clone(), exit)) {
                    stack.push((exit.mirror(), false));
                    stack.push((exit, true));
                    visited.insert((name.clone(), exit));
                }
            }
        }
        if let Some(item) = item {
            if !blacklist.contains(&item) {
                robot.item_pick(&item);
                continue;
            }
        }
        if let Some((compass, new_area)) = stack.pop() {
            robot.move_to(compass, new_area);
        } else {
            break;
        }
    }
}

fn go_to_security_checkpoint(robot: &mut Robot) {
    for &compass in &robot.path_to_checkpoint.clone() {
        robot.move_to(compass, false);
        room_info(&robot.output, robot.verbose);
    }
}

fn password_get(robot: &mut Robot) -> String {
    let all_items = robot.carrying;
    let num_items = all_items.count_ones();
    let item_names = robot.seen.clone();
    for items in 0..=all_items {
        robot.seen.clear();
        for item in 0..num_items {
            if (robot.carrying ^ items) & (1 << item) != 0 {
                if items & (1 << item) != 0 {
                    robot.item_pick(&item_names[item as usize]);
                } else {
                    robot.item_drop(&item_names[item as usize]);
                }
                room_info(&robot.output, robot.verbose);
            }
        }
        robot.carrying = items;
        robot.move_to(robot.last_move.unwrap(), false);
        let resp = room_info_lines(&robot.output, robot.verbose).join("\n");
        if resp.find("Alert!") == None {
            return resp.chars().filter(|c| c.is_numeric()).collect();
        }
    }
    panic!("Did not find the password!");
}

impl Solution for Day25 {
    fn part1(&self) -> String {
        let (input, sink) = channel();
        let output = exec(&self.program, sink, None);
        let mut robot = Robot {
            carrying: 0,
            last_move: None,
            seen: Items::new(),
            checkpoint_found: false,
            path_to_checkpoint: Vec::new(),
            input,
            output,
            verbose: self.verbose,
        };
        gather_items(&mut robot);
        go_to_security_checkpoint(&mut robot);
        password_get(&mut robot)
    }

    fn part2(&self) -> String {
        "".to_string()
    }
}

// State required to solve day 25
pub struct Day25 {
    program: Vec<Intcode>,
    verbose: bool,
}

pub fn solution(lines: Vec<&str>) -> Box<dyn Solution> {
    Box::new(Day25 {
        program: lines[0]
            .split(",")
            .map(|ic| ic.parse::<Intcode>().unwrap())
            .collect(),
        verbose: env::args().last().unwrap() == "-v",
    })
}
