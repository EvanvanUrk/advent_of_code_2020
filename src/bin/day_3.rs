extern crate advent_of_code_2020;

use self::advent_of_code_2020::*;

fn main() {
    let input = get_input("day_3.txt");
    let width = input.find('\n').unwrap() as u32;
    let height = input.matches('\n').count() as u32;
    let input = input.replace("\n", "");
    let map = Map::new(input, width, height);

    let slopes: [(u32, u32); 5] = [
        (1, 1),
        (1, 3),
        (1, 5),
        (1, 7),
        (2, 1),
    ];

    let mut tree_counts = Vec::with_capacity(5);

    for slope in &slopes[..] {
        tree_counts.push(map.get_trees_for_slope(*slope));
    }

    let product: u32 = tree_counts.iter().product();

    println!("{}", product);
}

struct Map {
    positions: String,
    width: u32,
    height: u32,
}

impl Map {
    fn new(map: String, width: u32, height: u32) -> Map {
        Map {
            positions: map,
            width,
            height
        }
    }

    fn get_position(&self, x: u32, y: u32) -> char {
        let idx = (x * self.width + y % self.width) as usize;
        self.positions.chars().nth(idx).unwrap()
    }

    fn get_trees_for_slope(&self, slope: (u32, u32)) -> u32 {
        let mut x = 0;
        let mut y = 0;
        let mut amount_of_trees = 0;
        while x < self.height {
            if self.get_position(x, y) == '#' {
                amount_of_trees += 1;
            }
            x += slope.0;
            y += slope.1;
        }
        amount_of_trees
    }
}
