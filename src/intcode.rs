use AddressMode::*;
use Instruction::*;
use num_derive::FromPrimitive;
use num_traits::*;
use std::collections::HashMap;
use std::sync::mpsc::*;
use std::thread;

pub type Intcode = i64;

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

fn to_mode(opcode: Intcode, position: Intcode) -> AddressMode {
    FromPrimitive::from_i32(opcode as i32 / pow(10, (position + 1) as usize) % 10).expect("Invalid mode")
}

fn to_instr(opcode: Intcode) -> Instruction {
    FromPrimitive::from_i32(opcode as i32 % 100).expect("Invalid instruction")
}

fn get(mem: &HashMap<Intcode, Intcode>, addr: Intcode) -> Intcode {
    *mem.get(&addr).unwrap_or(&0)
}

fn run(mem: &mut HashMap<Intcode, Intcode>, input: Receiver<Intcode>, output: Sender<Intcode>) {
    let mut ip: Intcode = 0;
    let mut relative_base: Intcode = 0;
    loop {
        let mut deferred_st = None;
        {
            let opcode = get(mem, ip);
            let ld = |offset| {
                let val = get(mem, ip + offset);
                match to_mode(opcode, offset) {
                    Position => get(mem, val),
                    Immediate => val,
                    Relative => get(mem, relative_base + val),
                }
            };
            let st = |offset, val| {
                let imm = get(mem, ip + offset);
                match to_mode(opcode, offset) {
                    Position => Some((val, imm)),
                    Immediate => panic!("Invalid store mode"),
                    Relative => Some((val, relative_base + imm)),
                }
            };
            let mut binop = |op: &dyn Fn(Intcode, Intcode) -> Intcode| {
                deferred_st = st(3, op(ld(1), ld(2)));
                ip + 4
            };
            let jmp_if = |pred: &dyn Fn(Intcode) -> bool| {
                if pred(ld(1)) {
                    ld(2)
                } else {
                    ip + 3
                }
            };
            ip = match to_instr(opcode) {
                Add => binop(&|a, b| a + b),
                Mul => binop(&|a, b| a * b),
                In => {
                    if let Ok(val) = input.recv() {
                        deferred_st = st(1, val);
                        ip + 2
                    } else {
                        break;
                    }
                }
                Out => {
                    output.send(ld(1)).unwrap();
                    ip + 2
                }
                JmpIfTrue => jmp_if(&|a| a != 0),
                JmpIfFalse => jmp_if(&|a| a == 0),
                LessThan => binop(&|a, b| if a < b { 1 } else { 0 }),
                Equals => binop(&|a, b| if a == b { 1 } else { 0 }),
                AdjustBase => {
                    relative_base = relative_base + ld(1);
                    ip + 2
                }
                Halt => break,
            }
        }
        // All immutable borrows must go out of scope before it is OK to store
        // to memory, so this kind of simulates "write-back" step in a CPU...
        if let Some((val, addr)) = deferred_st {
            mem.insert(addr, val);
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
    let mut memory = HashMap::new();
    for (addr, &val) in program.iter().enumerate() {
        memory.insert(addr as Intcode, val);
    }
    thread::spawn(move || {
        run(&mut memory, input, tx);
    });
    output
}
