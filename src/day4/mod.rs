use super::Solution;
use std::ops::RangeInclusive;

const PIN_DIGITS: usize = 6;
type Pin = u32;

// State required for solve day 4
pub struct State {
    candidate_pins: RangeInclusive<Pin>,
}

fn fill_digits(digits: &mut [u8; PIN_DIGITS], pin: &Pin, depth: usize) -> Pin {
    if depth < digits.len() {
        let d = fill_digits(digits, pin, depth + 1);
        digits[depth] = (d % 10) as u8;
        d / 10
    } else {
        *pin
    }
}

// Transforms 123456 into [1, 2, 3, 4, 5, 6]
fn to_digits(pin: &Pin) -> [u8; PIN_DIGITS] {
    let mut digits = [0; PIN_DIGITS];
    fill_digits(&mut digits, pin, 0);
    digits
}

fn has_pair(pin: &Pin) -> bool {
    to_digits(pin)
        .iter()
        .fold((false, 10), |acc, &digit| (acc.0 || acc.1 == digit, digit))
        .0
}

fn not_descending(pin: &Pin) -> bool {
    to_digits(pin)
        .iter()
        .fold((true, 0), |acc, &digit| (acc.0 && acc.1 <= digit, digit))
        .0
}

fn has_strict_pair(pin: &Pin) -> bool {
    to_digits(pin)
        .iter()
        .fold(&mut [0; 10], |cnt, &digit| {
            cnt[digit as usize] = cnt[digit as usize] + 1;
            cnt
        })
        .iter()
        .any(|&count| count == 2)
}

impl Solution for State {
    fn part1(&self) -> String {
        self.candidate_pins
            .clone()
            .filter(not_descending)
            .filter(has_pair)
            .count()
            .to_string()
    }

    fn part2(&self) -> String {
        self.candidate_pins
            .clone()
            .filter(not_descending)
            .filter(has_strict_pair)
            .count()
            .to_string()
    }
}

pub fn solution(lines: Vec<&str>) -> Box<dyn Solution> {
    let mut it = lines[0].split("-").map(|s| s.parse::<Pin>().unwrap());
    Box::new(State {
        candidate_pins: it.next().unwrap()..=it.next().unwrap(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d4_ex1() {
        assert!(has_pair(&111111));
    }

    #[test]
    fn d4_ex2() {
        assert!(has_pair(&223450));
    }

    #[test]
    fn d4_ex3() {
        assert!(!has_pair(&123789));
    }

    #[test]
    fn d4_ex4() {
        assert!(not_descending(&111111));
    }

    #[test]
    fn d4_ex5() {
        assert!(!not_descending(&223450));
    }

    #[test]
    fn d4_ex6() {
        assert!(not_descending(&123789));
    }

    #[test]
    fn d4_ex7() {
        assert!(has_strict_pair(&112233));
    }

    #[test]
    fn d4_ex8() {
        assert!(!has_strict_pair(&123444));
    }

    #[test]
    fn d4_ex9() {
        assert!(has_strict_pair(&111122));
    }
}
