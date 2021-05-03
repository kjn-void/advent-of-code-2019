use super::Solution;
use itertools::Itertools;
use std::collections::HashSet;

const WIDTH: isize = 5;
const HEIGHT: isize = 5;

type Area = u32;
type RecursiveArea = Vec<Area>;

fn bug_at(state: Area, x: isize, y: isize) -> u32 {
    if state & (1 << (x + y * WIDTH)) == 0 { 0 } else { 1 }
}

fn safe_bug_at(state: Area, x: isize, y: isize) -> u32 {
    if x < 0 || x >= WIDTH || y < 0 || y >= HEIGHT  {
        0
    } else {
        bug_at(state, x, y)
    }
}

fn next_state(state: Area) -> Area {
    let cnt_neigh = |x, y| {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .map(|(dx, dy)| safe_bug_at(state, x + dx, y + dy))
            .sum()
    };
    let mut new_state = 0;
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let num_neigh: u32 = cnt_neigh(x, y);
            if num_neigh == 1 || (bug_at(state, x, y) == 0 && num_neigh == 2) {
                new_state |= 1 << (x + y * WIDTH);
            }
        }
    }
    new_state
}

fn next_level_state(lvls: &[Area; 3]) -> Area {
    let mut new_state = 0;
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let num_neigh: u32 = match (x, y) {
                (0, 0) => vec![(0, 1, 2), (0, 2, 1), (1, 1, 0), (1, 0, 1)],
                (4, 0) => vec![(1, 3, 0), (0, 2, 1), (0, 3, 2), (1, 4, 1)],
                (0, 4) => vec![(0, 1, 2), (1, 0, 3), (1, 1, 4), (0, 2, 3)],
                (4, 4) => vec![(1, 3, 4), (1, 4, 3), (0, 3, 2), (0, 2, 3)],
                (2, 2) => vec![],
                (1, 2) => vec![(1, 0, 2), (1, 1, 1), (2, 0, 0), (2, 0, 1), (2, 0, 2), (2, 0, 3), (2, 0, 4), (1, 1, 3)],
                (2, 1) => vec![(1, 1, 1), (1, 2, 0), (1, 3, 1), (2, 0, 0), (2, 1, 0), (2, 2, 0), (2, 3, 0), (2, 4, 0)],
                (3, 2) => vec![(2, 4, 0), (2, 4, 1), (2, 4, 2), (2, 4, 3), (2, 4, 4), (1, 3, 1), (1, 4, 2), (1, 3, 3)],
                (2, 3) => vec![(1, 1, 3), (2, 0, 4), (2, 1, 4), (2, 2, 4), (2, 3, 4), (2, 4, 4), (1, 3, 3), (1, 2, 4)],
                (xm, 0) => vec![(1, xm - 1, 0), (0, 2, 1), (1, xm, 1), (1, xm + 1, 0)],
                (xm, 4) => vec![(1, xm - 1, 4), (1, xm, 3), (1, xm + 1, 4), (0, 2, 3)],
                (0, ym) => vec![(0, 1, 2), (1, 0, ym - 1), (1, 1, ym), (1, 0, ym + 1)],
                (4, ym) => vec![(1, 3, ym), (1, 4, ym - 1), (0, 3, 2), (1, 4, ym + 1)],
                (xm, ym) => vec![(1, xm - 1, ym), (1, xm, ym - 1), (1, xm + 1, ym), (1, xm, ym + 1)],
            }
            .iter()
            .map(|&(lvl, x, y)| bug_at(lvls[lvl], x, y))
            .sum();
            if num_neigh == 1 || (bug_at(lvls[1], x, y) == 0 && num_neigh == 2) {
                new_state |= 1 << (x + y * WIDTH);
            }
        }
    }
    new_state
}

fn next_recursive_state(state: RecursiveArea) -> RecursiveArea {
    let state_get = |level, dl: isize| {
        let eff_lvl = level as isize + dl;
        if eff_lvl < 0 || eff_lvl >= state.len() as isize {
            0
        } else {
            state[eff_lvl as usize]
        }
    };
    let mut new_state = Vec::new();
    let new_len = state.len() + 1;
    for level in 0..=new_len {
        let lvl_state = next_level_state(&[
            state_get(level, -2),
            state_get(level, -1),
            state_get(level, 0),
        ]);
        if lvl_state != 0 || level != 0 && level != new_len {
            new_state.push(lvl_state);
        }
    }
    new_state
}

fn live_bugs_after(seconds: u32, initial_state: Area) -> u32 {
    (0..seconds)
    .fold(vec![initial_state], |state, _| {
        next_recursive_state(state)
    })
    .iter()
    .map(|area: &Area| area.count_ones())
    .sum()
}

impl Solution for Day24 {
    fn part1(&self) -> String {
        let mut seen_states = HashSet::new();
        let mut state = self.initial_state;
        while !seen_states.contains(&state) {
            seen_states.insert(state);
            state = next_state(state);
        }
        state.to_string()
    }

    fn part2(&self) -> String {
        live_bugs_after(200, self.initial_state).to_string()
    }
}

// State required to solve day _
pub struct Day24 {
    initial_state: Area,
}

fn to_state(lines: &Vec<&str>) -> Area {
    let mut initial_state = 0;
    for (idx, tile) in lines
        .iter()
        .map(|s| s.to_string())
        .join("")
        .chars()
        .enumerate()
    {
        if tile == '#' {
            initial_state |= 1 << idx;
        }
    }
    initial_state
}

pub fn solution(lines: Vec<&str>) -> Box<dyn Solution> {
    Box::new(Day24 {
        initial_state: to_state(&lines),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d24_ex1() {
        let init_state = to_state(&vec!["....#", "#..#.", "#..##", "..#..", "#...."]);
        let state_1_min = to_state(&vec!["#..#.", "####.", "###.#", "##.##", ".##.."]);
        assert_eq!(next_state(init_state), state_1_min);
    }

    #[test]
    fn d24_ex2() {
        let input = vec!["....#", "#..#.", "#..##", "..#..", "#...."];
        assert_eq!(solution(input).part1(), "2129920");
    }

    #[test]
    fn d24_ex3() {
        let init_state = to_state(&vec!["....#", "#..#.", "#..##", "..#..", "#...."]);
        assert_eq!(live_bugs_after(10, init_state), 99);
    }

    #[test]
    fn d24_part1() {
        let input = vec!["####.", ".###.", ".#..#", "##.##", "###.."];
        assert_eq!(solution(input).part1(), "32511025");
    }


    #[test]
    fn d24_part2() {
        let input = vec!["####.", ".###.", ".#..#", "##.##", "###.."];
        assert_eq!(solution(input).part2(), "1932");
    }

}
