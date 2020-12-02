use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::path::Path;

fn main() {
    let path = Path::new("day_1.txt");
    let mut file = File::open(&path).expect("Error opening input file!");

    let mut s = String::new();
    file.read_to_string(&mut s).expect("Error reading input file!");

    let mut numbers: Vec<i32> = Vec::new();
    for string in s.split("\n") {
        match string.parse() {
            Err(_) => (),
            Ok(number) => numbers.push(number),
        }
    }

    let mut amount_of_entries = String::new();
    println!("How many entries are you looking for?");
    io::stdin().read_line(&mut amount_of_entries).expect("Failed to read input.");
    let amount_of_entries: usize = amount_of_entries.trim().parse().expect("Failed to parse input, please enter a number.");

    let mut sum = String::new();
    println!("What should the sum of the entries be?");
    io::stdin().read_line(&mut sum).expect("Failed to read input.");
    let sum: i32 = sum.trim().parse().expect("Failed to parse input, please enter a number.");

    let len = numbers.len() as i32;

    let counter = PermutationCounter::new(
        amount_of_entries,
        len
    );

    let mut entry_indices = None;
    for permutation in counter.into_iter() {
        let current_sum = permutation.iter().fold(0, |acc, index| {
            acc + numbers[*index as usize]
        });

        if current_sum == sum {
            entry_indices = Some(permutation);
            break;
        }
    }

    match entry_indices {
        None => println!("No {} entries in the input have the sum of {}", amount_of_entries, sum),
        Some(indices) => {
            let answer = indices.iter().fold(
                1,
                |acc, index| {
                    acc * numbers[*index as usize]
                }
            );

            println!("{}", answer);
        }
    }
}

struct PermutationCounter {
    index_max: i32,
    indices: Vec<i32>,
}

impl PermutationCounter {
    fn new(indices_amount: usize, index_max: i32) -> PermutationCounter {
        PermutationCounter {
            index_max,
            indices: vec![0; indices_amount],
        }
    }
}

impl Iterator for PermutationCounter {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.indices.last() {
            None => None,
            Some(last) => {
                if *last == self.index_max {
                    return None;
                }

                let mut counter = 0;
                loop {
                    self.indices[counter] += 1;
                    if self.indices[counter] == self.index_max {
                        if self.indices.len() == 1 {
                            return None;
                        }

                        self.indices[counter] = 0;
                    } else {
                        break;
                    }

                    counter += 1;
                    if counter == self.indices.len() {
                        break;
                    }
                }

                Some(self.indices.iter().cloned().collect())
            }
        }
    }
}
