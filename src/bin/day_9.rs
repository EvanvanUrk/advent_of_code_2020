extern crate advent_of_code_2020;

use self::advent_of_code_2020::*;

fn main() {
    let input = get_input("day_9.txt");

    let numbers: Vec<u64> = input.split('\n')
        .filter(|line| { line.len() > 0 })
        .map(|line| { line.parse().unwrap() })
        .collect();

    let mut invalid = 0;
    for idx in 25..numbers.len() {
        let mut prev: Vec<u64> = vec![0; 25];
        prev.copy_from_slice(&numbers[idx - 25..idx]);
        prev.sort();

        // cheat
        let min = prev[0] + prev[1];
        let max = prev[23] + prev[24];

        let cur = numbers[idx];

        if cur < min || cur > max {
            invalid = cur;
            break;
        }
    }

    println!("{}", invalid);

    let mut weakness = 0;
    let mut amount: usize = 2;
    'outer: while weakness == 0 {

        for idx in 0..numbers.len() - amount {
            let contiguous = &numbers[idx..idx+amount];
            let sum = contiguous.iter().fold(0, |acc, val| {
                acc + val
            });

            if sum == invalid {
                weakness = contiguous.iter().min().unwrap() + contiguous.iter().max().unwrap();
                break 'outer;
            }
        }

        amount += 1;
    }

    println!("{}", weakness);
}
