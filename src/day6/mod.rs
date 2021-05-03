use super::Solution;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

// Body centers, from outermost satellite body to world center body
fn centers(orbiting: &HashMap<String, String>, outermost: &String) -> Vec<String> {
    let mut satellite = Some(outermost);
    let mut cntrs = Vec::new();
    while let Some(sat) = satellite {
        cntrs.push(sat.clone());
        satellite = orbiting.get(sat);
    }
    cntrs
}

impl Solution for Day6 {
    fn part1(&self) -> String {
        let total_orbits: u32 = self
            .orbiting
            .values()
            .map(|sat| centers(&self.orbiting, sat).len() as u32)
            .sum();
        total_orbits.to_string()
    }

    fn part2(&self) -> String {
        let you = centers(&self.orbiting, &"YOU".to_string());
        let san = centers(&self.orbiting, &"SAN".to_string());
        let ys: HashSet<String> = HashSet::from_iter(you.into_iter().skip(1));
        let ss: HashSet<String> = HashSet::from_iter(san.into_iter().skip(1));
        ys.symmetric_difference(&ss).count().to_string()
    }
}

// State required to solve day 6
pub struct Day6 {
    orbiting: HashMap<String, String>,
}

pub fn solution(lines: Vec<&str>) -> Box<dyn Solution> {
    Box::new(Day6 {
        orbiting: lines
            .iter()
            .map(|line| {
                let mut s = line.split(")");
                let center = s.next().unwrap().to_string();
                let satellite = s.next().unwrap().to_string();
                (satellite, center)
            })
            .collect(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d6_ex1() {
        let input = vec!["COM)B", "B)C", "C)D"];
        assert!(solution(input).part1() == "6");
    }

    #[test]
    fn d6_ex2() {
        let input = vec![
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
        ];
        assert!(solution(input).part1() == "42");
    }

    #[test]
    fn d6_ex3() {
        let input = vec![
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "K)YOU",
            "I)SAN",
        ];
        assert!(solution(input).part2() == "4");
    }
}
