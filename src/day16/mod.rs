use super::Solution;
use itertools::Itertools;
use num_traits::pow;

const FFT_PATTERN: [i32; 4] = [0, 1, 0, -1];

#[derive(Clone, Copy, Debug)]
struct FftPattern {
    period: usize,
    rep: usize,
    idx: usize,
}

fn fft_pattern(period: usize) -> FftPattern {
    FftPattern {
        period: period,
        rep: 0,
        idx: 0,
    }
}

impl Iterator for FftPattern {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        if self.rep == 0 && self.idx == FFT_PATTERN.len() {
            None
        } else {
            let ret = FFT_PATTERN[self.idx];
            self.rep += 1;
            if self.rep == self.period {
                self.rep = 0;
                self.idx += 1;
            }
            Some(ret)
        }
    }
}

fn phase(input: &Vec<i32>) -> Vec<i32> {
    (1..=input.len())
        .map(|idx| {
            input
                .iter()
                .zip(fft_pattern(idx).cycle().skip(1))
                .map(|(&val, pat)| val * pat)
                .sum()
        })
        .map(|n: i32| n.abs() % 10)
        .collect()
}

impl Solution for State {
    fn part1(&self) -> String {
        (0..100)
            .fold(self.signal.clone(), |signal, _| phase(&signal))
            .iter()
            .take(8)
            .map(|d| d.to_string())
            .join("")
    }

    fn part2(&self) -> String {
        let mut real_signal = Vec::new();
        for _ in 0..10000 {
            let mut s = self.signal.clone();
            real_signal.append(&mut s);
        }
        let offset = real_signal
            .iter()
            .enumerate()
            .take(7)
            .map(|(i, &n)| pow(10, 6 - i) * n as usize)
            .sum();
        real_signal = real_signal.into_iter().skip(offset).collect();
        for _ in 0..100 {
            let mut tot = 0;
            for i in (0..real_signal.len()).rev() {
                tot += real_signal[i];
                real_signal[i] = tot.abs() % 10;
            }
        }
        real_signal.iter().take(8).join("")
    }
}

// State required to solve day 16
pub struct State {
    signal: Vec<i32>,
}

pub fn solution(lines: Vec<&str>) -> Box<dyn Solution> {
    Box::new(State {
        signal: lines[0]
            .chars()
            .map(|n| n.to_digit(10).unwrap() as i32)
            .collect(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d15_ex1() {
        let pattern: Vec<i32> = fft_pattern(1).cycle().take(8).collect();
        assert_eq!(pattern, vec![0, 1, 0, -1, 0, 1, 0, -1]);
    }

    #[test]
    fn d15_ex2() {
        let pattern: Vec<i32> = fft_pattern(2).take(8).collect();
        assert_eq!(pattern, vec![0, 0, 1, 1, 0, 0, -1, -1]);
    }

    #[test]
    fn d15_ex3() {
        let mut signal = vec![1, 2, 3, 4, 5, 6, 7, 8];
        signal = phase(&signal);
        assert_eq!(signal, vec![4, 8, 2, 2, 6, 1, 5, 8]);
        signal = phase(&signal);
        assert_eq!(signal, vec![3, 4, 0, 4, 0, 4, 3, 8]);
        signal = phase(&signal);
        assert_eq!(signal, vec![0, 3, 4, 1, 5, 5, 1, 8]);
        signal = phase(&signal);
        assert_eq!(signal, vec![0, 1, 0, 2, 9, 4, 9, 8]);
    }

    #[test]
    fn d15_ex4() {
        assert_eq!(
            solution(vec!["80871224585914546619083218645595"]).part1(),
            "24176176"
        );
    }

    #[test]
    fn d15_ex5() {
        assert_eq!(
            solution(vec!["19617804207202209144916044189917"]).part1(),
            "73745418"
        );
    }

    #[test]
    fn d15_ex6() {
        assert_eq!(
            solution(vec!["69317163492948606335995924319873"]).part1(),
            "52432133"
        );
    }

    #[test]
    fn d15_ex7() {
        assert_eq!(
            solution(vec!["03036732577212944063491565474664"]).part2(),
            "84462026"
        );
    }

    #[test]
    fn d15_ex8() {
        assert_eq!(
            solution(vec!["02935109699940807407585447034323"]).part2(),
            "78725270"
        );
    }

    #[test]
    fn d15_ex9() {
        assert_eq!(
            solution(vec!["03081770884921959731165446850517"]).part2(),
            "53553731"
        );
    }
}
