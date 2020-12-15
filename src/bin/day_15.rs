extern crate advent_of_code_2020;

use self::advent_of_code_2020::*;
use std::collections::HashMap;

fn main() {
    let input = get_input("day_15.txt");

    let mut numbers: Vec<usize> = input.split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let mut occurrences: HashMap<usize, Vec<usize>> = HashMap::new();

    for i in 0..30000000 {
        let spoken = if let Some(number) = numbers.get(i) {
            *number
        } else {
            let last_spoken = numbers.last().unwrap();
            let spoken = match occurrences.get(last_spoken) {
                Some(turns) => {
                    if turns.len() > 1 {
                        turns[turns.len() - 1] - turns[turns.len() - 2]
                    } else {
                        0
                    }
                },
                None => 0,
            };
            numbers.push(spoken);
            spoken
        };

        occurrences.entry(spoken).or_insert(Vec::new()).push(i + 1);
    }

    println!("{}", numbers.last().unwrap());
}
