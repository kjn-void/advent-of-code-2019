use regex::Regex;
use super::Solution;

type Deck = Vec<u32>;

enum Technique {
    DealIntoNewStack,
    DealWithIncrement(usize),
    Cut(isize),
}

fn deal_into_new_stack(deck: &Deck) -> Deck {
    deck.iter().rev().map(|&d| d).collect()
}

fn cut_n(deck: &Deck, n: isize) -> Deck {
    let cn = if n >= 0 {
        n as usize
    } else {
        deck.len() - n.abs() as usize
    };
    deck.into_iter()
        .cycle()
        .skip(cn)
        .take(deck.len())
        .map(|&d| d)
        .collect()
}

fn deal_with_increment(deck: &Deck, n: usize) -> Deck {
    let mut new_deck = (0..deck.len()).map(|_| 0).collect::<Vec<_>>();
    let mut idx = 0;
    for &d in deck {
        new_deck[idx] = d;
        idx = (idx + n) % deck.len();
    }
    new_deck
}

fn apply_techniques(techniques: &Vec<Technique>, deck: &Deck) -> Deck {
    techniques
        .iter()
        .fold(deck.clone(), |deck, technique| match technique {
            Technique::DealWithIncrement(n) => deal_with_increment(&deck, *n),
            Technique::DealIntoNewStack => deal_into_new_stack(&deck),
            Technique::Cut(n) => cut_n(&deck, *n),
        })
}

impl Solution for State {
    fn part1(&self) -> String {
        let deck = apply_techniques(&self.techniques, &(0..10007).collect());
        deck.iter().enumerate().filter(|(_, &val)| val==2019).next().unwrap().0.to_string()
    }

    fn part2(&self) -> String {
        "".to_string()
    }
}

// State required to solve day 22
pub struct State {
    techniques: Vec<Technique>,
}

pub fn solution(lines: Vec<&str>) -> Box<dyn Solution> {
    let re =
        Regex::new(r"^((deal into new stack)|(deal with increment (\d+))|(cut (-?\d+)))$").unwrap();
    let mut techniques = Vec::new();

    for line in lines {
        let caps = re.captures(line).unwrap();
        if let Some(_) = caps.get(2) {
            techniques.push(Technique::DealIntoNewStack);
        } else if let Some(inc) = caps.get(4) {
            techniques.push(Technique::DealWithIncrement(
                inc.as_str().parse::<usize>().unwrap(),
            ));
        } else {
            let n = caps.get(6).unwrap().as_str();
            techniques.push(Technique::Cut(n.parse::<isize>().unwrap()));
        }
    }
    Box::new(State { techniques })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d22_ex1() {
        let deck = (0..10).collect();
        assert_eq!(deal_into_new_stack(&deck), [9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    }

    #[test]
    fn d22_ex2() {
        let deck = (0..10).collect();
        assert_eq!(cut_n(&deck, 3), [3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);
    }

    #[test]
    fn d22_ex3() {
        let deck = (0..10).collect();
        assert_eq!(cut_n(&deck, -4), [6, 7, 8, 9, 0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn d22_ex4() {
        let deck = (0..10).collect();
        assert_eq!(
            deal_with_increment(&deck, 3),
            [0, 7, 4, 1, 8, 5, 2, 9, 6, 3]
        );
    }
}
