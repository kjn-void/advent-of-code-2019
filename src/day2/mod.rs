use super::Solution;
use Instruction::*;

type Intcode = usize;
type Addr = Intcode;

#[derive(PartialEq,Debug)]
enum Instruction {
    Add(Addr, Addr, Addr),
    Mul(Addr, Addr, Addr),
    Halt,
}

// State required for solve day 2
pub struct State {
    memory: Vec<Intcode>,
}

pub fn solution(lines: Vec<&str>) -> Box<dyn Solution> {
    Box::new(State {
        memory: lines[0].split(",").map(|ic|ic.parse::<Intcode>().unwrap()).collect()
    })
}

fn fetch(mem: &Vec<Intcode>, ip: Addr) -> Instruction {
    let opcode = mem[ip];
    match opcode {
         1 => Add(mem[ip+1], mem[ip+2], mem[ip+3]),
         2 => Mul(mem[ip+1], mem[ip+2], mem[ip+3]),
        99 => Halt,
         _ => panic!("Invalid opcode"),
    }
}

fn exec(instr: Instruction, mem: &mut Vec<Intcode>, ip: &mut Addr) -> bool {
    let mut new_ip = *ip + 4;
    match instr {
        Add(src0, src1, dst) => mem[dst] = mem[src0] + mem[src1],
        Mul(src0, src1, dst) => mem[dst] = mem[src0] * mem[src1],
        Halt => new_ip = *ip,
    };
    *ip = new_ip;
    instr == Halt
}

fn run(memory: &Vec<Intcode>, noun: Intcode, verb: Intcode) -> Intcode {
    let mut mem = memory.clone();
    let mut ip: Addr = 0;
    // The value placed in address 1 is called the noun, and the value 
    // placed in address 2 is called the verb
    mem[1] = noun;
    mem[2] = verb;
    loop {
        let instr = fetch(&mem, ip);
        if exec(instr, &mut mem, &mut ip) {
            break
        }
    }
    // Output is value at position 0 after the program halts.
    mem[0]
}

impl Solution for State {
    fn part1(&self) -> String {
        run(&self.memory, 12, 2).to_string()
    }

    fn part2(&self) -> String {
        let wanted_output = 19690720;
        let mut noun = 0;
        let mut verb = 0;
        loop {
            noun = noun + 1;
            if noun > 99 {
                noun = 0;
                verb = verb + 1;
            }
            if run(&self.memory, noun, verb) == wanted_output {
                break;
            }
        }
        (100 * noun + verb).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,13,1,19,1,19,10,23,\
        2,10,23,27,1,27,6,31,1,13,31,35,1,13,35,39,1,39,10,43,2,43,13,47,1,47,\
        9,51,2,51,13,55,1,5,55,59,2,59,9,63,1,13,63,67,2,13,67,71,1,71,5,75,2,\
        75,13,79,1,79,6,83,1,83,5,87,2,87,6,91,1,5,91,95,1,95,13,99,2,99,6,\
        103,1,5,103,107,1,107,9,111,2,6,111,115,1,5,115,119,1,119,2,123,1,6,\
        123,0,99,2,14,0,0";

     #[test]
    fn ex1() {
        assert!(solution(vec!["1,0,0,0,99,0,0,0,0,0,0,0,40"]).part1() == "42");
    }

    #[test]
    fn ex2() {
        assert!(solution(vec!["2,0,0,0,99,0,0,0,0,0,0,0,3"]).part1() == "6");
    }

    #[test]
    fn ex3() {
        assert!(solution(vec![INPUT]).part1() == "4090701");
    }

    #[test]
    fn ex4() {
        assert!(solution(vec![INPUT]).part2() == "6421");
    }
}
