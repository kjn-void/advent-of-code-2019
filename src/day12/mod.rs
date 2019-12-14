use super::Solution;
use num::integer::lcm;
use regex::Regex;
use std::cmp::Ordering;
use std::ops::Add;

type PlanetSystem = Vec<Moon>;

fn energy(planet_system: &PlanetSystem) -> i32 {
    planet_system.iter().fold(0, |energy, moon| {
        energy + moon.pos.energy() * moon.vel.energy()
    })
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Vec3 {
    v: [i32; 3],
}

fn binop(va: &Vec3, vb: &Vec3, op: &dyn Fn((&i32, &i32)) -> i32) -> Vec3 {
    let result: Vec<i32> = va.v.iter().zip(vb.v.iter()).map(op).collect();
    Vec3 {
        v: [result[0], result[1], result[2]],
    }
}

impl Vec3 {
    fn energy(&self) -> i32 {
        self.v.iter().fold(0, |e, coord| e + coord.abs())
    }

    fn c_cmp(&self, other: Vec3) -> Vec3 {
        binop(self, &other, &|(a, b)| match a.cmp(b) {
            Ordering::Less => -1,
            Ordering::Greater => 1,
            Ordering::Equal => 0,
        })
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        binop(&self, &other, &|(a, b)| a + b)
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3 { v: [0, 0, 0] }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Moon {
    pos: Vec3,
    vel: Vec3,
}

impl Moon {
    fn step_time(&self, moons: &PlanetSystem) -> Moon {
        let vel = moons
            .iter()
            .fold(self.vel, |new_vel, moon| new_vel + moon.pos.c_cmp(self.pos));
        Moon {
            pos: self.pos + vel,
            vel: vel,
        }
    }
}

fn step_time(moons: &PlanetSystem) -> PlanetSystem {
    let mut next_moons = Vec::new();
    for moon in moons {
        next_moons.push(moon.step_time(moons));
    }
    next_moons
}

// State required to solve day 12
pub struct State {
    moons: PlanetSystem,
}

impl Solution for State {
    fn part1(&self) -> String {
        energy(&(0..1000).fold(self.moons.clone(), |cur_moons, _| step_time(&cur_moons)))
            .to_string()
    }

    fn part2(&self) -> String {
        let chk = |moons: &PlanetSystem, get: &dyn Fn(Vec3) -> i32| {
            moons.iter().zip(self.moons.iter()).fold(true, |pred, m| {
                pred && get(m.0.vel) == 0 && get(m.0.pos) == get(m.1.pos)
            })
        };
        let mut result = [0; 3];
        let mut steps: u64 = 0;
        let mut moons = self.moons.clone();
        while result.iter().any(|&a| a == 0) {
            moons = step_time(&moons);
            steps = steps + 1;
            for dim in 0..result.len() {
                if result[dim] == 0 && chk(&moons, &|v: Vec3| v.v[dim]) {
                    result[dim] = steps;
                }
            }
        }
        lcm(result[0], lcm(result[1], result[2])).to_string()
    }
}

fn parse_input(lines: Vec<&str>) -> PlanetSystem {
    let re = Regex::new(r"<x=(?P<x>[-]?\d+), y=(?P<y>[-]?\d+), z=(?P<z>[-]?\d+)>").unwrap();
    let mut planet_system = Vec::new();
    for line in lines {
        let caps = re.captures(line).unwrap();
        planet_system.push(Moon {
            pos: Vec3 {
                v: [
                    caps["x"].parse::<i32>().unwrap(),
                    caps["y"].parse::<i32>().unwrap(),
                    caps["z"].parse::<i32>().unwrap(),
                ],
            },
            vel: Vec3::default(),
        });
    }
    planet_system
}

pub fn solution(lines: Vec<&str>) -> Box<dyn Solution> {
    Box::new(State {
        moons: parse_input(lines),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d12_ex1() {
        let moons = parse_input(vec![
            "<x=-1, y=0, z=2>",
            "<x=2, y=-10, z=-7>",
            "<x=4, y=-8, z=8>",
            "<x=3, y=5, z=-1>",
        ]);
        let step1 = step_time(&moons);
        assert_eq!(step1[0].pos, Vec3 { v: [2, -1, 1] });
        assert_eq!(step1[0].vel, Vec3 { v: [3, -1, -1] });
        let step2 = step_time(&step1);
        assert_eq!(step2[0].pos, Vec3 { v: [5, -3, -1] });
        assert_eq!(step2[0].vel, Vec3 { v: [3, -2, -2] });
    }

    #[test]
    fn d12_part1() {
        let lines = vec![
            "<x=-8, y=-10, z=0>",
            "<x=5, y=5, z=10>",
            "<x=2, y=-7, z=3>",
            "<x=9, y=-8, z=-3>",
        ];
        assert_eq!(solution(lines).part2(), "4686774924");
    }

    #[test]
    fn d12_part2() {
        let lines = vec![
            "<x=17, y=-9, z=4>",
            "<x=2, y=2, z=-13>",
            "<x=-1, y=5, z=-1>",
            "<x=4, y=7, z=-7>",
        ];
        assert_eq!(solution(lines).part2(), "537881600740876");
    }
}
