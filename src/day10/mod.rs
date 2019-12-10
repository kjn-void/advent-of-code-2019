use super::Solution;
use std::collections::HashSet;
use MapContent::*;

#[derive(PartialEq, Debug)]
enum MapContent {
    Space,
    Astroid,
}

type Coord = (i32, i32);

#[derive(Clone, Copy)]
struct Visible {
    pos: Coord,
    count: usize,
}

// State required to solve day 10
pub struct State {
    map: Vec<Vec<MapContent>>,
}

fn is_on_map(pos: Coord, width: i32, height: i32) -> bool {
    pos.0 >= 0 && pos.1 >= 0 && pos.0 < width && pos.1 < height
}

fn hit(
    map: &Vec<Vec<MapContent>>,
    removed: &HashSet<Coord>,
    origin: Coord,
    dir: Coord,
) -> Option<Coord> {
    let mut pos = origin;
    loop {
        pos.0 = pos.0 + dir.0;
        pos.1 = pos.1 + dir.1;
        if !is_on_map(pos, map[0].len() as i32, map.len() as i32) {
            return None;
        }
        if !removed.contains(&pos) && map[pos.1 as usize][pos.0 as usize] == Astroid {
            return Some(pos);
        }
    }
}

fn is_visible(map: &Vec<Vec<MapContent>>, from: Coord, to: Coord, dir: Coord) -> bool {
    if let Some(pos) = hit(map, &HashSet::new(), from, dir) {
        if pos == to {
            return true
        }
    }
    false
}


fn dir_vector(base: Coord, pos: Coord) -> Coord {
    let dx = pos.0 - base.0;
    let dy = pos.1 - base.1;
    let gcd = num::integer::gcd(dx, dy);
    (dx / gcd, dy / gcd)
}

fn count_visible(map: &Vec<Vec<MapContent>>, pos: Coord) -> usize {
    let mut count = 0;
    for row in 0..map.len() {
        let y = row as i32;
        for col in 0..map[row].len() {
            let x = col as i32;
            if map[row][col] == Astroid && (x != pos.0 || y != pos.1) {
                if is_visible(map, pos, (x, y), dir_vector(pos, (x, y))) {
                    count = count + 1;
                }
            }
        }
    }
    count
}

fn to_visibles(map: &Vec<Vec<MapContent>>) -> Vec<Visible> {
    let mut v = Vec::new();
    for row in 0..map.len() {
        let y = row as i32;
        for col in 0..map[row].len() {
            let x = col as i32;
            if map[row][col] == Astroid {
                v.push(Visible {
                    pos: (x, y),
                    count: count_visible(map, (x, y)),
                });
            }
        }
    }
    v
}

fn select_best_astroid(map: &Vec<Vec<MapContent>>) -> Visible {
    *to_visibles(map)
        .iter()
        .max_by(|a, b| a.count.cmp(&b.count))
        .unwrap()
}

fn angle(pos: Coord) -> f64 {
    let a = (-pos.1 as f64).atan2(-pos.0 as f64) * 180.0 / 3.1415;
    if a < 90.0 {
        a + 360.0
    } else {
        a
    }
}

fn unique_dir_vecs(map: &Vec<Vec<MapContent>>, laser_pos: Coord) -> Vec<Coord> {
    let mut dir_vecs = Vec::new();
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            let pos = (col as i32, row as i32);
            if map[row][col] == Astroid && laser_pos != pos {
                dir_vecs.push(dir_vector(laser_pos, pos));
            }
        }
    }
    dir_vecs.sort_by(|&a, &b| angle(a).partial_cmp(&angle(b)).unwrap());
    dir_vecs.dedup();
    dir_vecs
}

fn nth_destroyed(nth: usize, map: &Vec<Vec<MapContent>>, laser_pos: Coord) -> Coord {
    let mut removed = HashSet::new();
    let dir_vecs = unique_dir_vecs(&map, laser_pos);
    let mut n = nth;
    let mut dir_it = dir_vecs.iter().cycle();
    loop {
        let &dir = dir_it.next().unwrap();
        if let Some(pos) = hit(&map, &removed, laser_pos, dir) {
                removed.insert(pos);
            n = n - 1;
            if n == 0 {
                return pos;
            }
        }
    }
}

impl Solution for State {
    fn part1(&self) -> String {
        select_best_astroid(&self.map).count.to_string()
    }

    fn part2(&self) -> String {
        let p = select_best_astroid(&self.map).pos;
        let pos = nth_destroyed(200, &self.map, p);
        (pos.0 * 100 + pos.1).to_string()
    }
}

pub fn solution(lines: Vec<&str>) -> Box<dyn Solution> {
    Box::new(State {
        map: lines
            .iter()
            .map(|&row| {
                row.chars()
                    .map(|pos| if pos == '.' { Space } else { Astroid })
                    .collect()
            })
            .collect(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d10_ex1() {
        let map = vec![".#..#", ".....", "#####", "....#", "...##"];
        assert_eq!(solution(map).part1(), "8");
    }

    #[test]
    fn d10_ex2() {
        let map = vec![
            "......#.#.",
            "#..#.#....",
            "..#######.",
            ".#.#.###..",
            ".#..#.....",
            "..#....#.#",
            "#..#....#.",
            ".##.#..###",
            "##...#..#.",
            ".#....####",
        ];
        assert_eq!(solution(map).part1(), "33");
    }

    #[test]
    fn d10_ex3() {
        let map = vec![
            "#.#...#.#.",
            ".###....#.",
            ".#....#...",
            "##.#.#.#.#",
            "....#.#.#.",
            ".##..###.#",
            "..#...##..",
            "..##....##",
            "......#...",
            ".####.###.",
        ];
        assert_eq!(solution(map).part1(), "35");
    }

    #[test]
    fn d10_ex4() {
        let map = vec![
            ".#..#..###",
            "####.###.#",
            "....###.#.",
            "..###.##.#",
            "##.##.#.#.",
            "....###..#",
            "..#.#..#.#",
            "#..#.#.###",
            ".##...##.#",
            ".....#.#..",
        ];
        assert_eq!(solution(map).part1(), "41");
    }

    #[test]
    fn d10_ex5() {
        let map = vec![
            ".#..##.###...#######",
            "##.############..##.",
            ".#.######.########.#",
            ".###.#######.####.#.",
            "#####.##.#.##.###.##",
            "..#####..#.#########",
            "####################",
            "#.####....###.#.#.##",
            "##.#################",
            "#####.##.###..####..",
            "..######..##.#######",
            "####.##.####...##..#",
            ".#####..#.######.###",
            "##...#.##########...",
            "#.##########.#######",
            ".####.#.###.###.#.##",
            "....##.##.###..#####",
            ".#.#.###########.###",
            "#.#.#.#####.####.###",
            "###.##.####.##.#..##",
        ];
        assert_eq!(solution(map).part1(), "210");
    }
}
