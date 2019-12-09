use num_derive::FromPrimitive;
use num_traits::pow;
use num_traits::FromPrimitive;
use std::sync::mpsc::*;
use std::thread;
use AddressMode::*;
use Instruction::*;

pub type Intcode = i128;

#[derive(Debug, FromPrimitive, PartialEq)]
enum AddressMode {
    Position = 0,
    Immediate = 1,
    Relative = 2,
}

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
    AdjustBase = 9,
    Halt = 99,
}

fn to_mode(opcode: Intcode, position: usize) -> AddressMode {
    FromPrimitive::from_i32(opcode as i32 / pow(10, position + 1) % 10).expect("Invalid mode")
}

fn to_instr(opcode: Intcode) -> Instruction {
    FromPrimitive::from_i32(opcode as i32 % 100).expect("Invalid instruction")
}

fn run(mem: &mut Vec<Intcode>, input: Receiver<Intcode>, output: Sender<Intcode>) {
    let mut ip: usize = 0;
    let mut relative_base: usize = 0;
    loop {
        let mut st = None;
        {
            let opcode = mem[ip];
            let ld_imm = |offset| mem[ip + offset];
            let ld = |offset| {
                let val = ld_imm(offset);
                match to_mode(opcode, offset) {
                    Position => mem[val as usize],
                    Immediate => val,
                    Relative => mem[relative_base + val as usize],
                }
            };
            let mut binop = |op: &dyn Fn(Intcode, Intcode) -> Intcode| {
                st = Some((op(ld(1), ld(2)), ld_imm(3)));
                ip + 4
            };
            let jmp_if = |pred: &dyn Fn(Intcode) -> bool| {
                if pred(ld(1)) {
                    ld(2) as usize
                } else {
                    ip + 3
                }
            };
            ip = match to_instr(opcode) {
                Add => binop(&|a, b| a + b),
                Mul => binop(&|a, b| a * b),
                In => {
                    st = Some((input.recv().unwrap(), ld_imm(1)));
                    ip + 2
                }
                Out => {
                    output.send(ld(1)).unwrap();
                    ip + 2
                }
                JmpIfTrue => jmp_if(&|val| val != 0),
                JmpIfFalse => jmp_if(&|val| val == 0),
                LessThan => binop(&|a, b| if a < b { 1 } else { 0 }),
                Equals => binop(&|a, b| if a == b { 1 } else { 0 }),
                AdjustBase => {
                    relative_base = ld(1) as usize;
                    ip + 2
                }
                Halt => break,
            }
        }
        // All immutable borrows must go out of scope before it is OK to store
        // to 'mem', so this kind of simulates "write-back" step in a CPU...
        if let Some((val, addr)) = st {
            mem[addr as usize] = val;
        }
    }
}

pub fn exec(
    program: &Vec<Intcode>,
    input: Receiver<Intcode>,
    boot_output: Option<Intcode>,
) -> Receiver<Intcode> {
    let (tx, output) = channel();
    if let Some(bo) = boot_output {
        tx.send(bo).unwrap()
    }
    let mut memory = program.clone();
    thread::spawn(move || {
        run(&mut memory, input, tx);
    });
    output
}
