use super::intcode::*;
use super::Solution;
use std::sync::mpsc::*;

// State required to solve day _
pub struct State {
    program: Vec<Intcode>,
}

impl Solution for State {
    fn part1(&self) -> String {
        let (input, sink) = channel();
        let output = exec(&self.program, sink, None);
        input.send(1).unwrap();
        output.recv().unwrap().to_string()
    }

    fn part2(&self) -> String {
        let (input, sink) = channel();
        let output = exec(&self.program, sink, None);
        input.send(2).unwrap();
        output.recv().unwrap().to_string()
    }
}

pub fn solution(lines: Vec<&str>) -> Box<dyn Solution> {
    Box::new(State {
        program: lines[0]
            .split(",")
            .map(|ic| ic.parse::<Intcode>().unwrap())
            .collect(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d9_ex1() {
        let program = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let (_, sink) = channel();
        let output = exec(&program, sink, None);
        let mut result = Vec::new();
        while let Ok(r) = output.recv() {
            result.push(r);
        }
        assert_eq!(program, result);
    }

    #[test]
    fn d9_ex2() {
        let program = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let (_, sink) = channel();
        let output = exec(&program, sink, None);
        let result = output.recv().unwrap();
        assert_eq!(16, result.to_string().chars().count());
    }

    #[test]
    fn d9_ex3() {
        let program = vec![104, 1125899906842624, 99];
        let (_, sink) = channel();
        let output = exec(&program, sink, None);
        assert_eq!(1125899906842624, output.recv().unwrap());
    }
}
