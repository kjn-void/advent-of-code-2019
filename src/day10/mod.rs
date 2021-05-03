use super::vec2d::*;
use super::Solution;
use std::collections::HashSet;
use MapContent::*;

#[derive(PartialEq, Debug)]
enum MapContent {
    Space,
    Astroid,
}

#[derive(Clone, Copy)]
struct Visible {
    pos: Vec2D,
    count: usize,
}

fn is_on_map(pos: Vec2D, width: Coord, height: Coord) -> bool {
    pos >= Vec2D::default() && pos < Vec2D::from(width, height)
}

fn hit(
    map: &Vec<Vec<MapContent>>,
    removed: &HashSet<Vec2D>,
    origin: Vec2D,
    dir: Vec2D,
) -> Option<Vec2D> {
    let mut pos = origin;
    loop {
        pos += dir;
        if !is_on_map(pos, map[0].len() as i32, map.len() as i32) {
            return None;
        }
        if !removed.contains(&pos) && map[pos.y() as usize][pos.x() as usize] == Astroid {
            return Some(pos);
        }
    }
}

fn is_visible(map: &Vec<Vec<MapContent>>, from: Vec2D, to: Vec2D, dir: Vec2D) -> bool {
    if let Some(pos) = hit(map, &HashSet::new(), from, dir) {
        if pos == to {
            return true;
        }
    }
    false
}

fn dir_vector(base: Vec2D, pos: Vec2D) -> Vec2D {
    let dx = pos.x() - base.x();
    let dy = pos.y() - base.y();
    let gcd = num::integer::gcd(dx, dy);
    Vec2D::from(dx / gcd, dy / gcd)
}

fn count_visible(map: &Vec<Vec<MapContent>>, pos: Vec2D) -> usize {
    let mut count = 0;
    for row in 0..map.len() {
        let y = row as i32;
        for col in 0..map[row].len() {
            let x = col as i32;
            if map[row][col] == Astroid && (x != pos.x() || y != pos.y()) {
                let looking_at = Vec2D::from(x, y);
                if is_visible(map, pos, looking_at, dir_vector(pos, looking_at)) {
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
                let pos = Vec2D::from(x, y);
                v.push(Visible {
                    pos: pos,
                    count: count_visible(map, pos),
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

fn angle(pos: Vec2D) -> f64 {
    let a = (-pos.y() as f64).atan2(-pos.x() as f64) * 180.0 / 3.1415;
    if a < 90.0 {
        a + 360.0
    } else {
        a
    }
}

fn unique_dir_vecs(map: &Vec<Vec<MapContent>>, laser_pos: Vec2D) -> Vec<Vec2D> {
    let mut dir_vecs = Vec::new();
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            let pos = Vec2D::from(col as Coord, row as Coord);
            if map[row][col] == Astroid && laser_pos != pos {
                dir_vecs.push(dir_vector(laser_pos, pos));
            }
        }
    }
    dir_vecs.sort_by(|&a, &b| angle(a).partial_cmp(&angle(b)).unwrap());
    dir_vecs.dedup();
    dir_vecs
}

fn nth_destroyed(nth: usize, map: &Vec<Vec<MapContent>>, laser_pos: Vec2D) -> Vec2D {
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

impl Solution for Day10 {
    fn part1(&self) -> String {
        select_best_astroid(&self.map).count.to_string()
    }

    fn part2(&self) -> String {
        let p = select_best_astroid(&self.map).pos;
        let pos = nth_destroyed(200, &self.map, p);
        (pos.x() * 100 + pos.y()).to_string()
    }
}

// State required to solve day 10
pub struct Day10 {
    map: Vec<Vec<MapContent>>,
}

pub fn solution(lines: Vec<&str>) -> Box<dyn Solution> {
    Box::new(Day10 {
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
