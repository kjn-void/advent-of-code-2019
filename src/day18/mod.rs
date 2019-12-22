use super::vec2d::*;
use super::Solution;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

type Walls = HashSet<Vec2D>;
type Map = HashMap<Vec2D, u32>;
type KeyDistMatrix = HashMap<u32, Vec<KeyDistance>>;

#[derive(Debug)]
struct Robot {
    at_key: u32,    // key the robot currently standing at
    have_keys: u32, // bitfield, bit 1 << (key - 'a') is set of key is collected
    steps: Distance,
}

impl Eq for Robot {}

impl PartialEq for Robot {
    fn eq(&self, other: &Self) -> bool {
        self.steps == other.steps
    }
}

impl Ord for Robot {
    fn cmp(&self, other: &Self) -> Ordering {
        other.steps.cmp(&self.steps)
    }
}

impl PartialOrd for Robot {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.steps.cmp(&self.steps))
    }
}

#[derive(Debug)]
struct KeyDistance {
    steps: Distance,
    to: u32,    // key at the end of this edge
    keys: u32,  // bitfield, 1 << (key - 'a') is set if the key is along the path
    doors: u32, // bitfield, 1 << (door - 'A') is set if the door is along the path
}

fn has_item(item: u32, item_bag: u32) -> bool {
    (item & item_bag) != 0
}

fn is_key(tile: char) -> bool {
    tile.is_ascii_lowercase()
}

fn key(tile: char) -> u32 {
    1 << (tile as usize - 'a' as usize) as u32
}

fn is_door(tile: char) -> bool {
    tile.is_ascii_uppercase()
}

fn door(tile: char) -> u32 {
    1 << (tile as u32 - 'A' as u32)
}

// Returns the distance from 'key' to all other keys and what doors are along
// the route.
fn distances(origin: Vec2D, walls: &Walls, keys: &Map, doors: &Map) -> Vec<KeyDistance> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut edges = Vec::new();

    queue.push_back((origin, 0, 0, 0));
    while let Some((pos, steps, have_keys, seen_doors)) = queue.pop_front() {
        visited.insert(pos);
        let candidates = [UP, DOWN, LEFT, RIGHT]
            .iter()
            .map(|&dir| pos + dir)
            .filter(|new_pos| !visited.contains(new_pos) && !walls.contains(new_pos));
        for new_pos in candidates {
            let new_key = *keys.get(&new_pos).unwrap_or(&0);
            let new_steps = steps + 1;
            if new_key != 0 {
                edges.push(KeyDistance {
                    to: new_key,
                    steps: new_steps,
                    keys: have_keys,
                    doors: seen_doors,
                });
            }
            queue.push_back((
                new_pos,
                new_steps,
                have_keys | new_key,
                seen_doors | *doors.get(&new_pos).unwrap_or(&0),
            ))
        }
    }
    edges
}

fn dist_matrix(walls: &Walls, keys: &Map, doors: &Map) -> KeyDistMatrix {
    let mut hm = KeyDistMatrix::new();
    for (&pos, &key) in keys {
        hm.insert(key, distances(pos, walls, keys, doors));
    }
    hm
}

fn push_reachables(pqueue: &mut BinaryHeap<Robot>, robot: &Robot, edges: &KeyDistMatrix) {
    for edge in edges.get(&robot.at_key).unwrap() {
        let visited = has_item(edge.to, robot.have_keys);
        let can_unlock_doors = (!robot.have_keys & edge.doors) == 0;
        let all_keys_collected = (!robot.have_keys & edge.keys) == 0;
        let new_steps = edge.steps + robot.steps;

        if !visited && can_unlock_doors && all_keys_collected {
            pqueue.push(Robot {
                at_key: edge.to,
                have_keys: robot.have_keys | edge.to,
                steps: new_steps,
            });
        }
    }
}

fn collect_keys(start_pos: Vec2D, walls: &Walls, keys: &Map, doors: &Map) -> Distance {
    let all_keys = keys.values().fold(0, |keys, &key| keys | key);
    let dists = dist_matrix(walls, keys, doors);
    let mut pqueue = BinaryHeap::new();
    let mut duplicate_route = HashSet::new();

    // Add all keys reachable from the start position
    for edge in distances(start_pos, walls, keys, doors) {
        if edge.doors == 0 && edge.keys == 0 {
            pqueue.push(Robot {
                at_key: edge.to,
                have_keys: edge.to,
                steps: edge.steps,
            });
        }
    }

    let mut min_steps = std::u32::MAX;
    while let Some(robot) = pqueue.pop() {
        if robot.steps > min_steps {
            break;
        }
        if robot.have_keys == all_keys {
            min_steps = robot.steps;
        } else if duplicate_route.insert((robot.at_key, robot.have_keys)) {
            push_reachables(&mut pqueue, &robot, &dists);
        }
    }
    min_steps
}

impl Solution for State {
    fn part1(&self) -> String {
        collect_keys(self.start_pos, &self.walls, &self.keys, &self.doors).to_string()
    }

    fn part2(&self) -> String {
        let mut tot_distance = 0;
        // Shift map so that the original start position is at (0, 0), one
        // submap per quadrant
        let mut walls = self
            .walls
            .iter()
            .clone()
            .map(|&pos| pos - self.start_pos)
            .collect::<Walls>();
        walls.extend(&[UP, DOWN, LEFT, RIGHT]);

        for &start_pos in &[UP + LEFT, UP + RIGHT, DOWN + LEFT, DOWN + RIGHT] {
            let keys = self
                .keys
                .clone()
                .into_iter()
                .map(|(pos, key)| (pos - self.start_pos, key))
                .filter(|(pos, _)| pos.x() * start_pos.x() > 0 && pos.y() * start_pos.y() > 0)
                .collect::<Map>();
            let doors = self
                .doors
                .clone()
                .into_iter()
                .map(|(pos, door)| (pos - self.start_pos, door))
                .filter(|(pos, _)| pos.x() * start_pos.x() > 0 && pos.y() * start_pos.y() > 0)
                .filter(|(_, door)| keys.values().find(|&key| key == door) != None)
                .collect::<Map>();
            tot_distance += collect_keys(start_pos, &walls, &keys, &doors);
        }
        tot_distance.to_string()
    }
}

// State required to solve day 18
pub struct State {
    start_pos: Vec2D,
    walls: Walls,
    keys: Map,
    doors: Map,
}

pub fn solution(lines: Vec<&str>) -> Box<dyn Solution> {
    let mut keys = Map::new();
    let mut doors = Map::new();
    let mut walls = Walls::new();
    let mut start_pos = None;

    for (y, line) in lines.iter().enumerate() {
        for (x, tile) in line.chars().enumerate() {
            let pos = Vec2D::from(x as Coord, y as Coord);
            if tile == '@' {
                start_pos = Some(pos);
            } else if tile == '#' {
                walls.insert(pos);
            } else if is_key(tile) {
                keys.insert(pos, key(tile));
            } else if is_door(tile) {
                doors.insert(pos, door(tile));
            }
        }
    }
    Box::new(State {
        start_pos: start_pos.unwrap(),
        walls: walls,
        keys: keys,
        doors: doors,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d18_ex1() {
        let input = vec!["#########", "#b.A.@.a#", "#########"];
        assert_eq!(solution(input).part1(), "8");
    }

    #[test]
    fn d18_ex2() {
        let input = vec![
            "########################",
            "#f.D.E.e.C.b.A.@.a.B.c.#",
            "######################.#",
            "#d.....................#",
            "########################",
        ];
        assert_eq!(solution(input).part1(), "86");
    }

    #[test]
    fn d18_ex3() {
        let input = vec![
            "#################",
            "#i.G..c...e..H.p#",
            "########.########",
            "#j.A..b...f..D.o#",
            "########@########",
            "#k.E..a...g..B.n#",
            "########.########",
            "#l.F..d...h..C.m#",
            "#################",
        ];
        assert_eq!(solution(input).part1(), "136");
    }

    #[test]
    fn d18_ex4() {
        let input = vec![
            "########################",
            "#@..............ac.GI.b#",
            "###d#e#f################",
            "###A#B#C################",
            "###g#h#i################",
            "########################",
        ];
        assert_eq!(solution(input).part1(), "81");
    }
}
