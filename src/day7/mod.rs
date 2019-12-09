use permutohedron::LexicalPermutation;
use std::sync::mpsc::*;
use super::intcode::*;
use super::Solution;

const NUM_AMPS: usize = 5;

// State required to solve day 7
pub struct State {
    memory: Vec<Intcode>,
}

fn amplifiers(program: &Vec<Intcode>, phases: &[Intcode; NUM_AMPS]) -> Intcode {
    let (input, sink) = channel();
    let mut output = sink;
    input.send(phases[0]).unwrap();
    for i in 1..NUM_AMPS {
        output = exec(program, output, Some(phases[i]))
    }
    output = exec(program, output, None);
    let mut final_thrust = 0;
    input.send(0).unwrap();
    while let Ok(thrust) = output.recv() {
        final_thrust = thrust;
        if let Err(_) = input.send(thrust) {
            break;
        }
    }
    final_thrust
}

fn exec_with_phases(program: &Vec<Intcode>, phases: &mut [Intcode; NUM_AMPS]) -> Intcode {
    let mut thrusters_signal = Vec::new();
    loop {
        thrusters_signal.push(amplifiers(program, phases));
        if !phases.next_permutation() {
            break;
        }
    }
    thrusters_signal.into_iter().max().unwrap()
}

impl Solution for State {
    fn part1(&self) -> String {
        exec_with_phases(&self.memory, &mut [0, 1, 2, 3, 4]).to_string()
    }

    fn part2(&self) -> String {
        exec_with_phases(&self.memory, &mut [5, 6, 7, 8, 9]).to_string()
    }
}

pub fn solution(lines: Vec<&str>) -> Box<dyn Solution> {
    Box::new(State {
        memory: lines[0]
            .split(",")
            .map(|ic| ic.parse::<Intcode>().unwrap())
            .collect(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d7_ex1() {
        let memory = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        let phases = [4, 3, 2, 1, 0];
        assert_eq!(amplifiers(&memory, &phases), 43210);
    }

    #[test]
    fn d7_ex2() {
        let memory = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        let phases = [0, 1, 2, 3, 4];
        assert_eq!(amplifiers(&memory, &phases), 54321);
    }

    #[test]
    fn d7_ex3() {
        let memory = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        let phases = [1, 0, 4, 3, 2];
        assert_eq!(amplifiers(&memory, &phases), 65210);
    }

    #[test]
    fn d7_ex4() {
        let memory = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        let phases = [9, 8, 7, 6, 5];
        assert_eq!(amplifiers(&memory, &phases), 139629729);
    }

    #[test]
    fn d7_ex5() {
        let memory = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];
        let phases = [9, 7, 8, 5, 6];
        assert_eq!(amplifiers(&memory, &phases), 18216);
    }
}
