use super::intcode::*;
use super::Solution;
use std::env;
use std::sync::mpsc::*;

const SCENARIO_LEN: i32 = 17; // From description

#[derive(Debug, Copy, Clone)]
enum Op {
    AND,
    OR,
    NOT,
}

#[derive(Debug, Clone, Copy)]
enum Src {
    // T,
    J,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
}

#[derive(Debug, Copy, Clone)]
struct Instr {
    op: Op,
    src: Src,
}

type ScriptId = u32;
type Script = Vec<Instr>;
type Scenario = u32;
type Scenaions = Vec<Scenario>;

fn make_base_script(script_id: ScriptId, base_sreg: &Vec<Src>) -> Script {
    let mut sct = Script::new();
    let mut id = script_id;
    let mut srcs = base_sreg.clone();
    let mut src_idx;

    let (new_id, seed) = num_integer::div_rem(id, srcs.len() as u32);
    src_idx = seed as usize;
    id = new_id;
    sct.push(Instr {
        op: Op::OR,
        src: srcs[src_idx],
    });
    while id != 0 {
        srcs.remove(src_idx);
        let (new_id, seed) = num_integer::div_rem(id, 2 * srcs.len() as u32);
        id = new_id;
        src_idx = (seed >> 1) as usize;
        sct.push(Instr {
            op: if seed & 1 == 0 { Op::OR } else { Op::AND },
            src: srcs[src_idx],
        })
    }
    sct
}

fn make_scripts(script_id: ScriptId, base_sreg: &Vec<Src>) -> Vec<Script> {
    let base_script = make_base_script(script_id, base_sreg);
    let mut scts = Vec::new();

    // Generate all combinations where the generated instruction is or isn't
    // followed by "NOT J J".
    for i in 0..(2u32.pow(base_script.len() as u32)) {
        let mut script = Vec::new();
        for (j, &instr) in base_script.iter().enumerate() {
            script.push(instr);
            if (i >> j) & 1 != 0 {
                script.push(Instr {
                    op: Op::NOT,
                    src: Src::J,
                });
            }
        }
        scts.push(script);
    }
    scts
}

fn test_scenario(scenario: &Scenario, script: &Script) -> bool {
    let mut pos = 0;

    while pos < SCENARIO_LEN {
        let get_tile = |offset| scenario & (1u32 << (pos + offset)) != 0;
        if !get_tile(0) {
            return false;
        }
        let mut j = false;
        for instr in script {
            let s = match instr.src {
                Src::A => get_tile(1),
                Src::B => get_tile(2),
                Src::C => get_tile(3),
                Src::D => get_tile(4),
                Src::E => get_tile(5),
                Src::F => get_tile(6),
                Src::G => get_tile(7),
                Src::H => get_tile(8),
                Src::I => get_tile(9),
                Src::J => j,
            };
            match instr.op {
                Op::AND => j &= s,
                Op::OR => j |= s,
                Op::NOT => j = !s,
            }
        }
        pos += if j { 4 } else { 1 };
    }
    true
}

fn script_to_string(script: &Script, extended_range: bool) -> String {
    script
        .iter()
        .map(|instr| {
            let Instr { op, src } = instr;
            format!("{:?} {:?} J", op, src)
        })
        .collect::<Vec<_>>()
        .join("\n")
        + if extended_range {
            "\nRUN\n"
        } else {
            "\nWALK\n"
        }
}

fn get_hull_damage(program: &Vec<Intcode>, extended_range: bool, verbose: bool) -> u32 {
    let mut base_sregs = vec![Src::A, Src::B, Src::C, Src::D];
    let mut seed = 0;
    let mut scenarios = Scenaions::new();

    if extended_range {
        base_sregs.extend(&[Src::E, Src::F, Src::G, Src::H, Src::I]);
    }

    loop {
        let scripts = make_scripts(seed, &base_sregs);
        loop {
            let mut candidates = scripts.iter().filter(|script| {
                scenarios
                    .iter()
                    .all(|scenario| test_scenario(scenario, script))
            });
            if let Some(candidate) = candidates.next() {
                let (input, sink) = channel();
                let output = exec(&program.clone(), sink, None);
                for ch in script_to_string(candidate, extended_range).chars() {
                    input.send(ch as Intcode).unwrap();
                }
                loop {
                    let maybe_hull_damage = output.recv().unwrap();
                    if maybe_hull_damage > 127 {
                        if verbose {
                            println!("Scenarios needed : {}", scenarios.len());
                            println!("{}", script_to_string(candidate, extended_range));
                        }
                        return maybe_hull_damage as u32;
                    }
                    if maybe_hull_damage == '#' as Intcode {
                        break;
                    }
                }
                let mut new_scenario = 0;
                let mut idx = 1;
                while let Ok(tile) = output.recv() {
                    if tile == '\n' as Intcode {
                        break;
                    }
                    if tile == '.' as Intcode {
                        new_scenario |= 1 << idx;
                    }
                    idx += 1;
                }
                while let Ok(_) = output.recv() {}
                scenarios.push(!new_scenario);
            } else {
                break;
            }
        }
        seed += 1;
    }
}

impl Solution for State {
    fn part1(&self) -> String {
        get_hull_damage(&self.program, false, self.verbose).to_string()
    }

    fn part2(&self) -> String {
        get_hull_damage(&self.program, true, self.verbose).to_string()
    }
}

// State required to solve day 21
pub struct State {
    program: Vec<Intcode>,
    verbose: bool,
}

pub fn solution(lines: Vec<&str>) -> Box<dyn Solution> {
    Box::new(State {
        program: lines[0]
            .split(",")
            .map(|ic| ic.parse::<Intcode>().unwrap())
            .collect(),
        verbose: env::args().last().unwrap() == "-v",
    })
}
