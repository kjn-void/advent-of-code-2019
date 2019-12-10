use super::Solution;

// State required to solve day _
pub struct State {}

impl Solution for State {
    fn part1(&self) -> String {
        "".to_string()
    }

    fn part2(&self) -> String {
        "".to_string()
    }
}

pub fn solution(lines: Vec<&str>) -> Box<dyn Solution> {
    Box::new(State {})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(solution(vec![""]).part1(), "");
    }
}
