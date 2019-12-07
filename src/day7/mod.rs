use super::Solution;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use permutohedron::LexicalPermutation;
use Instruction::*;
use ProgState::*;

type Intcode = i32;
const NUM_AMPS: usize = 5;

#[derive(Debug, FromPrimitive, PartialEq)]
enum Instruction {
    Add = 1,
    Mul = 2,
    In = 3,
    Out = 4,
    JmpIfTrue = 5,
    JmpIfFalse = 6,
    LessThan = 7,
    Equals = 8,
    Halt = 99,
}

#[derive(Copy, Clone, Debug)]
enum ProgState {
    Resume(usize),
    Done,
}

// State required to solve day 7
pub struct State {
    memory: Vec<Intcode>,
}

fn opcode_to_instr(opcode: Intcode) -> Instruction {
    FromPrimitive::from_i32(opcode % 100).expect("Invalid instruction")
}

fn run(
    mem: &mut Vec<Intcode>,
    input: &mut dyn Iterator<Item = Intcode>,
    first_ip: usize,
    last_output: Intcode,
) -> (Intcode, ProgState) {
    let mut ip: usize = first_ip;
    loop {
        let mut st = None;
        {
            let opcode = mem[ip];
            let is_imm = [
                false,
                opcode / 100 % 10 != 0,
                opcode / 1000 % 10 != 0,
                opcode / 10000 % 10 != 0,
            ];
            let ld_imm = |offset| mem[ip + offset as usize];
            let ld = |offset| {
                if is_imm[offset] {
                    ld_imm(offset)
                } else {
                    mem[ld_imm(offset) as usize]
                }
            };
            ip = match opcode_to_instr(opcode) {
                Add => {
                    st = Some((ld(1) + ld(2), ld_imm(3)));
                    ip + 4
                }
                Mul => {
                    st = Some((ld(1) * ld(2), ld_imm(3)));
                    ip + 4
                }
                In => {
                    st = Some((input.next().unwrap(), ld_imm(1)));
                    ip + 2
                }
                Out => return (ld(1), Resume(ip + 2)),
                JmpIfTrue => {
                    if ld(1) != 0 {
                        ld(2) as usize
                    } else {
                        ip + 3
                    }
                }
                JmpIfFalse => {
                    if ld(1) == 0 {
                        ld(2) as usize
                    } else {
                        ip + 3
                    }
                }
                LessThan => {
                    st = Some((if ld(1) < ld(2) { 1 } else { 0 }, ld_imm(3)));
                    ip + 4
                }
                Equals => {
                    st = Some((if ld(1) == ld(2) { 1 } else { 0 }, ld_imm(3)));
                    ip + 4
                }
                Halt => return (last_output, Done),
            }
        }
        // All immutable borrows must go out of scope before it is OK to store
        // to 'mem', so this kind of simulates "write-back" step in a CPU...
        if let Some((val, addr)) = st {
            mem[addr as usize] = val;
        }
    }
}

fn amplifiers(program: &Vec<Intcode>, phases: &[Intcode; NUM_AMPS]) -> Intcode {
    let mut mems: Vec<Vec<Intcode>> = phases.iter().map(|_| program.clone()).collect();
    let mut ips = [Resume(0); NUM_AMPS];
    let mut outputs = [0; NUM_AMPS];
    let mut result = 0;
    let mut first = true;
    'terminated: loop {
        for (idx, &phase) in phases.iter().enumerate() {
            if let Resume(ip) = ips[idx] {
                let input = if first {
                    vec![phase, result]
                } else {
                    vec![result]
                };
                let res = run(&mut mems[idx], &mut input.into_iter(), ip, outputs[idx]);
                result = res.0;
                ips[idx] = res.1;
                outputs[idx] = result;
            } else {
                break 'terminated;
            }
        }
        first = false;
    }
    result
}

fn run_with_phases(program: &Vec<Intcode>, phases: &mut [Intcode; NUM_AMPS]) -> Intcode {
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
        run_with_phases(&self.memory, &mut [0, 1, 2, 3, 4]).to_string()
    }

    fn part2(&self) -> String {
        run_with_phases(&self.memory, &mut [5, 6, 7, 8, 9]).to_string()
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
