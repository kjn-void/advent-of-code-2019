use super::vec2d::*;
use super::Solution;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

type Map = HashMap<Vec2D, char>;
type KeyDistMatrix = Vec<Vec<KeyDistance>>;

#[derive(Debug)]
struct Keykeeper {
    tile: char,     // current tile
    have_keys: u32, // bitfield, bit 1 << (key - 'a') is set of key is collected
    steps: Distance,
}

impl Eq for Keykeeper {}

impl PartialEq for Keykeeper {
    fn eq(&self, other: &Self) -> bool {
        self.steps == other.steps
    }
}

impl Ord for Keykeeper {
    fn cmp(&self, other: &Self) -> Ordering {
        other.steps.cmp(&self.steps)
    }
}

impl PartialOrd for Keykeeper {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.steps.cmp(&self.steps))
    }
}

#[derive(Debug)]
struct KeyDistance {
    to: char,
    steps: Distance,
    keys: u32,  // bitfield, 1 << (door - 'A') is set if the key is along the path
    doors: u32, // bitfield, 1 << (door - 'A') is set if the door is along the path
}

fn all_keys(map: &Map) -> u32 {
    map.values()
        .filter(|&&tile| is_key(tile))
        .fold(0, |keys, &tile| keys | key(tile))
}

fn has_item(item: u32, item_bag: u32) -> bool {
    (item & item_bag) != 0
}

fn is_key(tile: char) -> bool {
    tile.is_ascii_lowercase()
}

fn key(key: char) -> u32 {
    1 << key_idx(key) as u32
}

fn key_idx(key: char) -> usize {
    (key as usize - 'a' as usize)
}

fn is_door(tile: char) -> bool {
    tile.is_ascii_uppercase()
}

fn door(door: char) -> u32 {
    1 << (door as u32 - 'A' as u32)
}

fn get_pos(map: &Map, what: char) -> Vec2D {
    *map.iter()
        .filter(|(_, &tile)| tile == what)
        .next()
        .unwrap()
        .0
}

fn get_keys(map: &Map) -> Vec<char> {
    map.values()
        .filter(|&&tile| is_key(tile))
        .map(|&key| key)
        .collect()
}

// Returns the distance from 'key' to all other keys and what doors are along
// the route.
fn distances(map: &Map, key_tile: char) -> Vec<KeyDistance> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut dists = Vec::new();

    queue.push_back((get_pos(map, key_tile), 0, 0, 0));
    while let Some((pos, steps, keys, doors)) = queue.pop_front() {
        visited.insert(pos);
        let candidates = [UP, DOWN, LEFT, RIGHT]
            .iter()
            .map(|&dir| {
                let new_pos = dir + pos;
                (new_pos, map.get(&new_pos).unwrap())
            })
            .filter(|(new_pos, &tile)| tile != '#' && !visited.contains(new_pos));
        for (new_pos, &tile) in candidates {
            let is_key = is_key(tile);
            let new_keys = keys | if is_key { key(tile) } else { 0 };
            let new_doors = doors | if is_door(tile) { door(tile) } else { 0 };
            let new_steps = steps + 1;

            if is_key {
                dists.push(KeyDistance {
                    to: tile,
                    steps: new_steps,
                    keys: keys,
                    doors: doors,
                });
            }
            queue.push_back((new_pos, new_steps, new_keys, new_doors))
        }
    }
    dists
}

fn push_reachables(pq: &mut BinaryHeap<Keykeeper>, kk: &Keykeeper, dists: &KeyDistMatrix) {
    for dist in &dists[key_idx(kk.tile)] {
        let new_key = key(dist.to);
        let visited = has_item(new_key, kk.have_keys);
        let can_unlock_doors = (!kk.have_keys & dist.doors) == 0;
        let all_keys_collected = (!kk.have_keys & dist.keys) == 0;
        let new_steps = dist.steps + kk.steps;

        // Not visited, is currently reachable and already collected all keys
        // along the path
        if !visited && can_unlock_doors && all_keys_collected {
            pq.push(Keykeeper {
                tile: dist.to,
                have_keys: kk.have_keys | new_key,
                steps: new_steps,
            });
        }
    }
}

fn dist_matrix(map: &Map) -> KeyDistMatrix {
    let mut m = Vec::new();
    for _ in 0..26 {
        m.push(Vec::new());
    }
    for &key_tile in &get_keys(map) {
        m[key_idx(key_tile)] = distances(map, key_tile);
    }
    m
}

fn collect_keys(map: &Map) -> Distance {
    let all_keys = all_keys(map);
    let dists = dist_matrix(map);
    let mut pq = BinaryHeap::new();
    let mut duplicate_route = HashSet::new();

    // Add all keys reachable from the start position
    for dist in distances(map, '@') {
        if is_key(dist.to) && dist.doors == 0 && dist.keys == 0 {
            pq.push(Keykeeper {
                tile: dist.to,
                have_keys: key(dist.to),
                steps: dist.steps,
            });
        }
    }

    let mut min_steps = std::u32::MAX;
    while let Some(kk) = pq.pop() {
        if kk.steps > min_steps {
            break
        }
        if kk.have_keys == all_keys {
            min_steps = kk.steps;
        } else if duplicate_route.insert((kk.tile, kk.have_keys)) {
            push_reachables(&mut pq, &kk, &dists);
        }
    }
    min_steps
}

impl Solution for State {
    fn part1(&self) -> String {
        collect_keys(&self.map).to_string()
    }

    fn part2(&self) -> String {
        "".to_string()
    }
}

// State required to solve day 18
pub struct State {
    map: Map,
}

pub fn solution(lines: Vec<&str>) -> Box<dyn Solution> {
    let mut map = Map::new();
    for y in 0..lines.len() {
        for (x, ch) in lines[y].chars().enumerate() {
            map.insert(Vec2D::from(x as Coord, y as Coord), ch);
        }
    }
    Box::new(State { map: map })
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
