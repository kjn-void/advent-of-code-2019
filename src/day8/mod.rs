use super::Solution;
use itertools::Itertools;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

const BLACK: char = '0';
const TRANSPARENT: char = '2';

fn render(layers: &Vec<String>, width: usize, height: usize) -> Vec<String> {
    let mut image = Vec::new();
    for h in 0..height {
        let mut row = String::from("");
        for w in 0..width {
            let mut pixel = TRANSPARENT;
            for layer in layers {
                if pixel == TRANSPARENT {
                    pixel = layer.chars().nth(h * width + w).unwrap();
                    if pixel != TRANSPARENT {
                        pixel = if pixel == BLACK { '▒' } else { '█' }
                    }
                }
            }
            row.push(pixel)
        }
        image.push(row)
    }
    image
}

fn count_pixels(layer: &String, pixel: char) -> usize {
    layer.chars().filter(|&pxl| pxl == pixel).count()
}

impl Solution for State {
    fn part1(&self) -> String {
        let zeros: Vec<usize> = self
            .layers
            .iter()
            .map(|layer| count_pixels(layer, '0'))
            .collect();
        let min_zeros = zeros.iter().min().unwrap();
        let selected_layer = &self.layers[zeros.iter().position(|cnt| cnt == min_zeros).unwrap()];
        (count_pixels(&selected_layer, '1') * count_pixels(&selected_layer, '2')).to_string()
    }

    fn part2(&self) -> String {
        let msg = render(&self.layers, WIDTH, HEIGHT);
        "\n".to_string() + &msg.join("\n")
    }
}

// State required to solve day 8
pub struct State {
    layers: Vec<String>,
}

pub fn solution(lines: Vec<&str>) -> Box<dyn Solution> {
    let mut layers = Vec::new();
    for layer in &lines[0].chars().chunks(WIDTH * HEIGHT) {
        layers.push(layer.collect());
    }
    Box::new(State { layers: layers })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d8_ex1() {
        let layers = vec!["0222", "1122", "2212", "0000"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(
            render(&layers, 2, 2),
            vec!["▒█".to_string(), "█▒".to_string()]
        );
    }
}
