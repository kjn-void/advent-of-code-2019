use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod intcode;
mod vec2d;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

type Day = u32;

pub trait Solution {
    // Solves first part of the problem
    fn part1(&self) -> String;
    // Solves second part of the problem
    fn part2(&self) -> String;
}

fn solution_get(day: Day, input: &mut dyn BufRead) -> Box<dyn Solution> {
    let storage: Vec<String> = input.lines().map(|line| line.unwrap()).collect();
    let lines = storage.iter().map(|s| s as &str).collect();
    match day {
        1 => day1::solution(lines),
        2 => day2::solution(lines),
        3 => day3::solution(lines),
        4 => day4::solution(lines),
        5 => day5::solution(lines),
        6 => day6::solution(lines),
        7 => day7::solution(lines),
        8 => day8::solution(lines),
        9 => day9::solution(lines),
        10 => day10::solution(lines),
        11 => day11::solution(lines),
        12 => day12::solution(lines),
        13 => day13::solution(lines),
        14 => day14::solution(lines),
        15 => day15::solution(lines),
        16 => day16::solution(lines),
        17 => day17::solution(lines),
        18 => day18::solution(lines),
        19 => day19::solution(lines),
        20 => day20::solution(lines),
        21 => day21::solution(lines),
        22 => day22::solution(lines),
        23 => day23::solution(lines),
        24 => day24::solution(lines),
        25 => day25::solution(lines),
        _ => panic!("Invalid day specified"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} DAY INPUT_FILE", args[0])
    } else {
        let day = args[1].parse::<Day>().unwrap();
        let f = if args.len() == 2 || args[2] == "-v" {
            File::open(format!("src/day{}/input.txt", day))
        } else {
            File::open(&args[2])
        }.expect("Failed to open input file");
        let mut input = BufReader::new(f);
        let solution = solution_get(day, &mut input);
        let start = std::time::Instant::now();
        println!("🕯️  Part 1 : {}", solution.part1());
        println!("🕯️  Part 2 : {}", solution.part2());
        println!("⌚ Took   : {} ms", start.elapsed().as_millis());
    }
}
