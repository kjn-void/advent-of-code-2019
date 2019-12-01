use super::Solution;

// State required for solving day 2
pub struct State {
}

pub fn solution(lines: Vec<&str>) -> Box<dyn Solution> {
    Box::new(State {  })
}

impl Solution for State {
    fn part1(&self) -> String {
	"".to_string()
    }

    fn part2(&self) -> String {
	"".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
    }
}
