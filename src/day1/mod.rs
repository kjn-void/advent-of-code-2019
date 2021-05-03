use super::Solution;

type Mass = i32;
type Fuel = i32;

// State required to solve day 1
pub struct Day1 {
    modules: Vec<Mass>,
}

// Fuel required to launch with a specific mass
fn fuel_required(mass: &Mass) -> Fuel {
    mass / 3 - 2
}

// Fuel required to launch with a specific mass when fuel itself got mass
fn fuel_with_mass_required(mass: &Mass) -> Fuel {
    let fuel = fuel_required(mass);

    if fuel <= 0 {
        0
    } else {
        fuel + fuel_with_mass_required(&fuel)
    }
}

impl Solution for Day1 {
    fn part1(&self) -> String {
        let fuel: Fuel = self.modules.iter().map(fuel_required).sum();
        fuel.to_string()
    }

    fn part2(&self) -> String {
        let fuel: Fuel = self.modules.iter().map(fuel_with_mass_required).sum();
        fuel.to_string()
    }
}

pub fn solution(lines: Vec<&str>) -> Box<dyn Solution> {
    Box::new(Day1 {
        modules: lines
            .iter()
            .map(|line| line.parse::<Mass>().unwrap())
            .collect(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d1_ex1() {
        assert!(solution(vec!["12"]).part1() == "2");
    }

    #[test]
    fn d1_ex2() {
        assert!(solution(vec!["14"]).part1() == "2");
    }

    #[test]
    fn d1_ex3() {
        assert!(solution(vec!["1969"]).part1() == "654");
    }

    #[test]
    fn d1_ex4() {
        assert!(solution(vec!["100756"]).part1() == "33583");
    }

    #[test]
    fn d1_ex5() {
        assert!(solution(vec!["12"]).part2() == "2");
    }

    #[test]
    fn d1_ex6() {
        assert!(solution(vec!["1969"]).part2() == "966");
    }

    #[test]
    fn d1_ex7() {
        assert!(solution(vec!["100756"]).part2() == "50346");
    }
}
