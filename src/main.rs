use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod day1;
mod day2;
mod day3;
mod day4;

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
        _ => panic!("Invalid day specified"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} DAY INPUT_FILE", args[0])
    } else {
        let day = args[1].parse::<Day>().unwrap();
        let f = File::open(&args[2]).expect("Failed to open input file");
        let mut input = BufReader::new(f);
        let solution = solution_get(day, &mut input);

        println!("🕯️ Part 1: {}", solution.part1());
        println!("🕯️ Part 2: {}", solution.part2());
    }
}
