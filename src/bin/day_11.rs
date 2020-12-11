extern crate advent_of_code_2020;

use self::advent_of_code_2020::*;
use State::*;
use std::fmt;

fn main() {
    let input = get_input("day_11.txt");

    let width = input.find('\n').unwrap();
    let height = input.matches('\n').count() + 1;
    let input = input.replace("\n", "");

    let input: Vec<State> = input.chars()
        .map(|c| {                
            match c {
                '#' => Occupied,
                'L' => Empty,
                '.' => Floor,
                _ => panic!("Unknown character {} encountered", c),
            }
        })
        .collect();

    let mut grid = Grid::new(input.clone(), width as i32, height as i32);
    while grid.apply_rules(false, 4) {}
    println!("{}", grid.get_occupied());

    let mut grid = Grid::new(input, width as i32, height as i32);
    while grid.apply_rules(true, 5) {}
    println!("{}", grid.get_occupied());
}

#[derive(Clone, PartialEq)]
enum State {
    Occupied,
    Empty,
    Floor,
}

struct Grid {
    grid: Vec<State>,
    width: i32,
    height: i32,
}

impl Grid {
    fn new(grid: Vec<State>, width: i32, height: i32) -> Grid {
        Grid {
            grid,
            width,
            height
        }
    }

    fn get_occupied(&self) -> usize {
        self.grid.iter()
            .filter(|&x| x == &Occupied)
            .count()
    }

    fn index(&self, x: i32, y: i32) -> usize {
        (y * self.width + x % self.width) as usize
    }

    fn get(&self, x: i32, y: i32) -> Option<&State> {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return None;
        }

        self.grid.get(self.index(x, y))
    }

    fn apply_rules(&mut self, visible: bool, limit: u8) -> bool {
        let mut new_grid = self.grid.clone();
        let mut changed = false;
        for x in 0..self.width {
            for y in 0..self.height {
                if let Some(state) = self.get(x, y) {
                    match state {
                        Empty => {
                            if self.get_surrounding_occupied(x, y, visible) == 0 {
                                new_grid[self.index(x, y)] = Occupied;
                                changed = true;
                            }
                        },
                        Occupied => {
                            if self.get_surrounding_occupied(x, y, visible) >= limit {
                                new_grid[self.index(x, y)] = Empty;
                                changed = true;
                            }
                        },
                        _ => {},
                    }
                }
            }
        }

        self.grid = new_grid;

        changed
    }

    fn get_surrounding_occupied(&self, x: i32, y: i32, visible: bool) -> u8 {
        let mut occupied = 0;
        for &i in [-1, 0, 1].iter() {
            for &j in [-1, 0, 1].iter() {
                if i == 0 && j == 0 {
                    continue;
                }

                let mut x = x + i;
                let mut y = y + j;

                if visible {
                    while let Some(state) = self.get(x, y) {
                        match state {
                            Occupied => {
                                occupied += 1;
                                break;
                            },
                            Empty => break,
                            Floor => {
                                x += i;
                                y += j;
                            },
                        }
                    }
                } else {
                    if let Some(state) = self.get(x, y) {
                        match state {
                            Occupied => occupied += 1,
                            _ => {},
                        }
                    }
                }
            }
        }

        occupied
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.grid.as_slice().chunks(self.width as usize) {
            for state in line {
                let symbol = match state {
                    Occupied => '#',
                    Empty => 'L',
                    Floor => '.',
                };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
