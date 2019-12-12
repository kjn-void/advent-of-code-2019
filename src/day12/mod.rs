use super::Solution;
use primes::factors_uniq;
use regex::Regex;
use std::ops::Add;
use std::cmp::Ordering;

type PlanetSystem = Vec<Moon>;

fn energy(planet_system: &PlanetSystem) -> i32 {
    planet_system.iter().fold(0, |energy, moon| {
        energy + moon.pos.energy() * moon.vel.energy()
    })
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Vec3 {
    fn energy(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    fn c_cmp(&self, other: Vec3) -> Vec3 {
        let cmp = |a: i32, b: i32| match a.cmp(&b) {
            Ordering::Less => -1,
            Ordering::Greater => 1,
            Ordering::Equal => 0,
        };
        Vec3 {
            x: cmp(self.x, other.x),
            y: cmp(self.y, other.y),
            z: cmp(self.z, other.z),
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3 { x: 0, y: 0, z: 0 }
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
        let get_x = |v: Vec3| v.x;
        let get_y = |v: Vec3| v.y;
        let get_z = |v: Vec3| v.z;
        let chk = |moons: &PlanetSystem, get: &dyn Fn(Vec3) -> i32| {
            moons.iter().zip(self.moons.iter()).fold(true, |pred, m| {
                pred && get(m.0.vel) == 0 && get(m.0.pos) == get(m.1.pos)
            })
        };
        let mut x = 0;
        let mut y = 0;
        let mut z = 0;
        let mut steps: u64 = 0;
        let mut moons = self.moons.clone();
        while x == 0 || y == 0 || z == 0 {
            moons = step_time(&moons);
            steps = steps + 1;
            if x == 0 && chk(&moons, &get_x) {
                x = steps;
            }
            if y == 0 && chk(&moons, &get_y) {
                y = steps;
            }
            if z == 0 && chk(&moons, &get_z) {
                z = steps;
            }
        }
        loop {
            let fact = factors_uniq(x * y * z);
            if let Some(denom) = fact
                .iter()
                .filter(|&d| x % d == 0 && y % d == 0 && z % d == 0)
                .last()
            {
                x = x / denom;
                y = y / denom;
                z = z / denom;
            } else {
                break;
            }
        }
        (x * y * z).to_string()
    }
}

fn parse_input(lines: Vec<&str>) -> PlanetSystem {
    let re = Regex::new(r"<x=(?P<x>[-]?\d+), y=(?P<y>[-]?\d+), z=(?P<z>[-]?\d+)>").unwrap();
    let mut planet_system = Vec::new();
    for line in lines {
        let caps = re.captures(line).unwrap();
        planet_system.push(Moon {
            pos: Vec3 {
                x: caps["x"].parse::<i32>().unwrap(),
                y: caps["y"].parse::<i32>().unwrap(),
                z: caps["z"].parse::<i32>().unwrap(),
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
    fn ex1() {
        let moons = parse_input(vec![
            "<x=-1, y=0, z=2>",
            "<x=2, y=-10, z=-7>",
            "<x=4, y=-8, z=8>",
            "<x=3, y=5, z=-1>",
        ]);
        let step1 = step_time(&moons);
        assert_eq!(step1[0].pos, Vec3 { x: 2, y: -1, z: 1 });
        assert_eq!(step1[0].vel, Vec3 { x: 3, y: -1, z: -1 });
        let step2 = step_time(&step1);
        assert_eq!(step2[0].pos, Vec3 { x: 5, y: -3, z: -1 });
        assert_eq!(step2[0].vel, Vec3 { x: 3, y: -2, z: -2 });
    }
}
