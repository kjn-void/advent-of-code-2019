use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod intcode;
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
        let start = std::time::Instant::now();
        println!("🕯️  Part 1 : {}", solution.part1());
        println!("🕯️  Part 2 : {}", solution.part2());
        println!("⌚ Took   : {} ms", start.elapsed().as_millis());
    }
}
