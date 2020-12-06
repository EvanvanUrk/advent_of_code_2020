extern crate advent_of_code_2020;

use self::advent_of_code_2020::*;

fn main() {
    let input = get_input("day_6.txt");

    let part_1: usize = input.trim().split("\n\n").into_iter()
        .map(|group| {
            let mut chars: Vec<char> = group.trim().replace("\n", "").chars().collect();
            chars.sort();
            chars.dedup();
            chars.len()
        })
        .sum();

    println!("{}", part_1);

    let part_2: usize = input.trim().split("\n\n").into_iter()
        .map(|group| {
            let people = group.matches("\n").count() + 1;
            let mut chars: Vec<char> = group.trim().replace("\n", "").chars().collect();
            chars.sort();
            chars.dedup();
            chars.iter()
                .map(|c| {
                    group.matches(*c).count()
                })
                .filter(|count| {
                    *count == people
                })
                .count()
        })
        .sum();

    println!("{}", part_2);
}
