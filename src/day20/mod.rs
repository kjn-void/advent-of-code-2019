use super::vec2d::*;
use super::Solution;
use std::collections::{HashMap, HashSet, VecDeque};

type Passage = HashSet<Vec2D>;
type Teleports = HashMap<Vec2D, (Vec2D, i32)>;

fn bfs(
    start_pos: Vec2D,
    end_pos: Vec2D,
    passage: &Passage,
    teleports: &Teleports,
    recur: i32,
) -> Distance {
    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();
    let dirs = [UP, DOWN, LEFT, RIGHT];
    // Add first step from entrance
    queue.extend(
        dirs.iter()
            .map(|&dir| start_pos + dir)
            .filter(|new_pos| passage.get(&new_pos) != None)
            .inspect(|&new_pos| { visited.insert((new_pos, 0), 0); })
            .map(|new_pos| (new_pos, 0, 0)),
    );

    while let Some((pos, steps, level)) = queue.pop_front() {
        // render(&visited, passage, teleports, pos, level);
        for new_pos in dirs.iter().map(|&dir| pos + dir) {
            if visited.get(&(new_pos, level)) == None {
                // Exit only avaiable at the outermost level
                visited.insert((pos, level), steps);
                if new_pos == end_pos && level == 0 {
                    return steps;
                }
                if passage.get(&new_pos) != None {
                    if let Some(&(tele_pos, dl)) = teleports.get(&new_pos) {
                        // Outside teleports cannot be used in part 2 in level 0
                        if level + dl * recur >= 0 {
                            queue.push_back((tele_pos, steps + 1, level + dl * recur));
                        }
                    } else {
                        queue.push_back((new_pos, steps + 1, level));
                    }
                }
            }
        }
    }
    panic!("No solution");
}

impl Solution for State {
    fn part1(&self) -> String {
        bfs(
            self.start_pos,
            self.end_pos,
            &self.passage,
            &self.teleports,
            0,
        )
        .to_string()
    }

    fn part2(&self) -> String {
        bfs(
            self.start_pos,
            self.end_pos,
            &self.passage,
            &self.teleports,
            1,
        )
        .to_string()
    }
}

// State required to solve day 20
pub struct State {
    start_pos: Vec2D,
    end_pos: Vec2D,
    passage: Passage,
    teleports: Teleports,
}

// Checks if a teleporter is on the outside or the inside, returns if the level
// should be decreased (outside) or increasaed (inside)
fn dl(pos: Vec2D, width: Coord, height: Coord) -> i32 {
    if pos.x() < 2 || pos.x() > width - 2 || pos.y() < 2 || pos.y() > height - 2 {
        -1
    } else {
        1
    }
}

pub fn solution(lines: Vec<&str>) -> Box<dyn Solution> {
    let mut passage = Passage::new();
    let mut map = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, tile) in line.chars().enumerate() {
            let pos = Vec2D::from(x as Coord, y as Coord);
            map.insert(pos, tile);
            if tile == '.' {
                passage.insert(pos);
            }
        }
    }
    let (width, height) = map.keys().fold((0, 0), |max, pos| {
        (
            if max.0 >= pos.x() { max.0 } else { pos.x() },
            if max.1 >= pos.y() { max.1 } else { pos.y() },
        )
    });
    let dl = |pos| dl(pos, width as Coord, height as Coord);
    let mut teleports = Teleports::new();
    let mut tdst: HashMap<(char, char), (Vec2D, Vec2D)> = HashMap::new();
    let mut start_pos = None;
    let mut end_pos = None;

    for y in 1..=height {
        for x in 1..=width {
            let pos = Vec2D::from(x as Coord, y as Coord);
            let l = *map.get(&(pos + LEFT)).unwrap_or(&' ');
            let r = *map.get(&pos).unwrap_or(&' ');
            let u = *map.get(&(pos + UP)).unwrap_or(&' ');
            let d = *map.get(&pos).unwrap_or(&' ');
            let (f, s, tel_pos) = if l.is_ascii_uppercase() && r.is_ascii_uppercase() {
                (
                    l,
                    r,
                    if map.get(&(pos + RIGHT)).unwrap_or(&' ') == &'.' {
                        (pos, pos + RIGHT)
                    } else {
                        (pos + LEFT, pos + LEFT + LEFT)
                    },
                )
            } else if u.is_ascii_uppercase() && d.is_ascii_uppercase() {
                (
                    u,
                    d,
                    if map.get(&(pos + DOWN)).unwrap_or(&' ') == &'.' {
                        (pos, pos + DOWN)
                    } else {
                        (pos + UP, pos + UP + UP)
                    },
                )
            } else {
                (' ', ' ', (Vec2D::default(), Vec2D::default()))
            };

            if (f, s) != (' ', ' ') {
                if (f, s) == ('A', 'A') {
                    start_pos = Some(tel_pos.0);
                } else if (f, s) == ('Z', 'Z') {
                    end_pos = Some(tel_pos.0);
                } else if let Some(&dst_pos) = tdst.get(&(f, s)) {
                    teleports.insert(dst_pos.0, (tel_pos.1, dl(dst_pos.0)));
                    teleports.insert(tel_pos.0, (dst_pos.1, dl(tel_pos.0)));
                    passage.insert(dst_pos.0);
                    passage.insert(tel_pos.0);
                } else {
                    tdst.insert((f, s), tel_pos);
                }
            }
        }
    }
    Box::new(State {
        start_pos: start_pos.unwrap(),
        end_pos: end_pos.unwrap(),
        passage: passage,
        teleports: teleports,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d20_ex1() {
        let input = vec![
            "         A         ",
            "         A         ",
            "  #######.#########",
            "  #######.........#",
            "  #######.#######.#",
            "  #######.#######.#",
            "  #######.#######.#",
            "  #####  B    ###.#",
            "BC...##  C    ###.#",
            "  ##.##       ###.#",
            "  ##...DE  F  ###.#",
            "  #####    G  ###.#",
            "  #########.#####.#",
            "DE..#######...###.#",
            "  #.#########.###.#",
            "FG..#########.....#",
            "  ###########.#####",
            "             Z     ",
            "             Z     ",
        ];
        assert_eq!(solution(input).part1(), "23");
    }

    #[test]
    fn d20_ex2() {
        let input = vec![
            "                   A               ",
            "                   A               ",
            "  #################.#############  ",
            "  #.#...#...................#.#.#  ",
            "  #.#.#.###.###.###.#########.#.#  ",
            "  #.#.#.......#...#.....#.#.#...#  ",
            "  #.#########.###.#####.#.#.###.#  ",
            "  #.............#.#.....#.......#  ",
            "  ###.###########.###.#####.#.#.#  ",
            "  #.....#        A   C    #.#.#.#  ",
            "  #######        S   P    #####.#  ",
            "  #.#...#                 #......VT",
            "  #.#.#.#                 #.#####  ",
            "  #...#.#               YN....#.#  ",
            "  #.###.#                 #####.#  ",
            "DI....#.#                 #.....#  ",
            "  #####.#                 #.###.#  ",
            "ZZ......#               QG....#..AS",
            "  ###.###                 #######  ",
            "JO..#.#.#                 #.....#  ",
            "  #.#.#.#                 ###.#.#  ",
            "  #...#..DI             BU....#..LF",
            "  #####.#                 #.#####  ",
            "YN......#               VT..#....QG",
            "  #.###.#                 #.###.#  ",
            "  #.#...#                 #.....#  ",
            "  ###.###    J L     J    #.#.###  ",
            "  #.....#    O F     P    #.#...#  ",
            "  #.###.#####.#.#####.#####.###.#  ",
            "  #...#.#.#...#.....#.....#.#...#  ",
            "  #.#####.###.###.#.#.#########.#  ",
            "  #...#.#.....#...#.#.#.#.....#.#  ",
            "  #.###.#####.###.###.#.#.#######  ",
            "  #.#.........#...#.............#  ",
            "  #########.###.###.#############  ",
            "           B   J   C               ",
            "           U   P   P               ",
        ];
        assert_eq!(solution(input).part1(), "58");
    }

    #[test]
    fn d20_ex3() {
        let input = vec![
            "             Z L X W       C                 ",
            "             Z P Q B       K                 ",
            "  ###########.#.#.#.#######.###############  ",
            "  #...#.......#.#.......#.#.......#.#.#...#  ",
            "  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  ",
            "  #.#...#.#.#...#.#.#...#...#...#.#.......#  ",
            "  #.###.#######.###.###.#.###.###.#.#######  ",
            "  #...#.......#.#...#...#.............#...#  ",
            "  #.#########.#######.#.#######.#######.###  ",
            "  #...#.#    F       R I       Z    #.#.#.#  ",
            "  #.###.#    D       E C       H    #.#.#.#  ",
            "  #.#...#                           #...#.#  ",
            "  #.###.#                           #.###.#  ",
            "  #.#....OA                       WB..#.#..ZH",
            "  #.###.#                           #.#.#.#  ",
            "CJ......#                           #.....#  ",
            "  #######                           #######  ",
            "  #.#....CK                         #......IC",
            "  #.###.#                           #.###.#  ",
            "  #.....#                           #...#.#  ",
            "  ###.###                           #.#.#.#  ",
            "XF....#.#                         RF..#.#.#  ",
            "  #####.#                           #######  ",
            "  #......CJ                       NM..#...#  ",
            "  ###.#.#                           #.###.#  ",
            "RE....#.#                           #......RF",
            "  ###.###        X   X       L      #.#.#.#  ",
            "  #.....#        F   Q       P      #.#.#.#  ",
            "  ###.###########.###.#######.#########.###  ",
            "  #.....#...#.....#.......#...#.....#.#...#  ",
            "  #####.#.###.#######.#######.###.###.#.#.#  ",
            "  #.......#.......#.#.#.#.#...#...#...#.#.#  ",
            "  #####.###.#####.#.#.#.#.###.###.#.###.###  ",
            "  #.......#.....#.#...#...............#...#  ",
            "  #############.#.#.###.###################  ",
            "               A O F   N                     ",
            "               A A D   M                     ",
        ];
        assert_eq!(solution(input).part2(), "396");
    }
}
