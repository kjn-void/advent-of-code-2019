use super::intcode::*;
use super::Solution;
use std::env;
use std::sync::mpsc::channel;

impl Solution for Day23 {
    fn part1(&self) -> String {
        let mut nics = Vec::new();
        for addr in 0..50 {
            let (input, sink) = channel();
            let output = exec(&self.program, sink, None);
            input.send(addr as Intcode).unwrap();
            nics.push((input, output));
        }
        'net: loop {
            for nic in &nics {
                if let Ok(dst) = nic.1.try_recv() {
                    let x = nic.1.recv().unwrap();
                    let y = nic.1.recv().unwrap();
                    if dst == 255 {
                        break 'net y;
                    }
                    nics[dst as usize].0.send(x).unwrap();
                    nics[dst as usize].0.send(y).unwrap();
                } else {
                    nic.0.send(-1).unwrap();
                }
            }
        }
        .to_string()
    }

    fn part2(&self) -> String {
        let mut nics = Vec::new();
        for addr in 0..50 {
            let (input, sink) = channel();
            let output = exec(&self.program, sink, None);
            input.send(addr as Intcode).unwrap();
            nics.push((input, output));
        }
        let mut nat_x = -1;
        let mut nat_y = -1;
        'net: loop {
            let mut is_idle = true;
            for nic in nics.iter() {
                if let Ok(dst) = nic.1.recv_timeout(std::time::Duration::from_micros(100)) {
                    let x = nic.1.recv().unwrap();
                    let y = nic.1.recv().unwrap();
                    if dst == 255 {
                        if self.verbose {
                            println!("{}", y);
                        }
                        if nat_y == y {
                            break 'net y;
                        }
                        nat_x = x;
                        nat_y = y;
                    } else {
                        is_idle = false;
                        nics[dst as usize].0.send(x).unwrap();
                        nics[dst as usize].0.send(y).unwrap();
                    }
                } else {
                    nic.0.send(-1).unwrap();
                }
            }
            if is_idle {
                nics[0].0.send(nat_x).unwrap();
                nics[0].0.send(nat_y).unwrap();
                std::thread::sleep(std::time::Duration::from_micros(1));
            }
        }
        .to_string()
    }
}

// State required to solve day _
pub struct Day23 {
    program: Vec<Intcode>,
    verbose: bool,
}

pub fn solution(lines: Vec<&str>) -> Box<dyn Solution> {
    Box::new(Day23 {
        program: lines[0]
            .split(",")
            .map(|ic| ic.parse::<Intcode>().unwrap())
            .collect(),
        verbose: env::args().last().unwrap() == "-v",
    })
}
