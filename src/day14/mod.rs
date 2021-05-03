use super::Solution;
use regex::Regex;
use std::collections::HashMap;

type Count = u64;

#[derive(Clone, Debug)]
struct Chemical {
    name: String,
    units: Count,
}

type Syntheses = HashMap<String, Vec<Chemical>>;
type Inventory = HashMap<String, Count>;

fn get_chemical(chemical: &Chemical, syntheses: &Syntheses, inventory: &mut Inventory) -> Count {
    if chemical.name == "ORE" {
        // Unlimited supply of ore
        return chemical.units;
    }
    let mut cnt = 0;
    loop {
        let inv = *inventory.get(&chemical.name).unwrap_or(&0);
        if chemical.units <= inv {
            // Use left-over from earlier synthesis
            inventory.insert(chemical.name.clone(), inv - chemical.units);
            return cnt
        }
        let syn = syntheses.get(&chemical.name).unwrap();
        let res = syn.last().unwrap();
        // Number of times this synthesis should run to get sufficient reagent
        let syn_mul = (chemical.units - inv + res.units - 1) / res.units;
        for reagent in &syn[0..(syn.len() - 1)] {
            let mut r = reagent.clone();
            r.units = r.units * syn_mul;
            cnt = cnt + get_chemical(&r, syntheses, inventory);
        }
        inventory.insert(chemical.name.clone(), inv + syn_mul * res.units);
    }
}

fn ore_per_n_fuel(n: Count, syntheses: &Syntheses) -> Count {
    let fuel = Chemical {
        name: "FUEL".to_string(),
        units: n,
    };
    let mut inventory = HashMap::new();
    get_chemical(&fuel, syntheses, &mut inventory)
}

impl Solution for Day14 {
    fn part1(&self) -> String {
        ore_per_n_fuel(1, &self.syntheses).to_string()
    }

    fn part2(&self) -> String {
        let mut lo = 1;
        let mut hi = 10;
        let ore = 1000000000000;
        while ore_per_n_fuel(hi, &self.syntheses) < ore {
            hi = hi * 10;
        }
        loop {
            let mid = (lo + hi) / 2;
            let this_ore = ore_per_n_fuel(mid, &self.syntheses);
            if hi - lo <= 1 {
                break lo
            }
            if this_ore < ore {
                lo = mid;
            } else {
                hi = mid
            }
        }.to_string()
    }
}

// State required to solve day 14
pub struct Day14 {
    syntheses: Syntheses,
}

pub fn solution(lines: Vec<&str>) -> Box<dyn Solution> {
    let re = Regex::new(r"((\d+) ([[:alpha:]]+))(, | => |$)").unwrap();
    let mut syntheses = HashMap::new();
    for line in &lines {
        let mut chemicals: Vec<Chemical> = Vec::new();
        for cap in re.captures_iter(line) {
            let mut i = 1;
            while i < cap.len() {
                chemicals.push(Chemical {
                    name: cap[i + 2].to_string(),
                    units: cap[i + 1].parse::<Count>().unwrap(),
                });
                i = i + 4;
            }
        }
        syntheses.insert(chemicals.last().unwrap().name.clone(), chemicals);
    }
    Box::new(Day14 {
        syntheses: syntheses,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d14_ex1() {
        let input = vec![
            "10 ORE => 10 A",
            "1 ORE => 1 B",
            "7 A, 1 B => 1 C",
            "7 A, 1 C => 1 D",
            "7 A, 1 D => 1 E",
            "7 A, 1 E => 1 FUEL",
        ];
        assert_eq!(solution(input).part1(), "31");
    }

    #[test]
    fn d14_ex2() {
        let input = vec![
            "9 ORE => 2 A",
            "8 ORE => 3 B",
            "7 ORE => 5 C",
            "3 A, 4 B => 1 AB",
            "5 B, 7 C => 1 BC",
            "4 C, 1 A => 1 CA",
            "2 AB, 3 BC, 4 CA => 1 FUEL",
        ];
        assert_eq!(solution(input).part1(), "165");
    }

    #[test]
    fn d14_ex3() {
        let input = vec![
            "157 ORE => 5 NZVS",
            "165 ORE => 6 DCFZ",
            "44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL",
            "12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ",
            "179 ORE => 7 PSHF",
            "177 ORE => 5 HKGWZ",
            "7 DCFZ, 7 PSHF => 2 XJWVT",
            "165 ORE => 2 GPVTF",
            "3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
        ];
        assert_eq!(solution(input).part1(), "13312");
    }

    #[test]
    fn d14_ex4() {
        let input = vec![
            "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG",
            "17 NVRVD, 3 JNWZP => 8 VPVL",
            "53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL",
            "22 VJHF, 37 MNCFX => 5 FWMGM",
            "139 ORE => 4 NVRVD",
            "144 ORE => 7 JNWZP",
            "5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC",
            "5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV",
            "145 ORE => 6 MNCFX",
            "1 NVRVD => 8 CXFTF",
            "1 VJHF, 6 MNCFX => 4 RFSQX",
            "176 ORE => 6 VJHF",
        ];
        assert_eq!(solution(input).part1(), "180697");
    }

    #[test]
    fn d14_ex5() {
        let input = vec![
            "171 ORE => 8 CNZTR",
            "7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL",
            "114 ORE => 4 BHXH",
            "14 VRPVC => 6 BMBT",
            "6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL",
            "6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT",
            "15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW",
            "13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW",
            "5 BMBT => 4 WPTQ",
            "189 ORE => 9 KTJDG",
            "1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP",
            "12 VRPVC, 27 CNZTR => 2 XDBXC",
            "15 KTJDG, 12 BHXH => 5 XCVML",
            "3 BHXH, 2 VRPVC => 7 MZWV",
            "121 ORE => 7 VRPVC",
            "7 XCVML => 6 RJRHP",
            "5 BHXH, 4 VRPVC => 5 LTCX",
        ];
        assert_eq!(solution(input).part1(), "2210736");
    }
}
