extern crate advent_of_code_2020;

use self::advent_of_code_2020::*;

fn main() {
    let input = get_input("day_13.txt");

    let lines: Vec<&str> = input.split('\n')
        .filter(|&l| l.len() > 0)
        .collect();

    let size = lines[1].matches(',').count();

    let pattern: Vec<(usize, usize)> = lines[1].split(',')
        .enumerate()
        .filter(|id| id.1 != "x")
        .map(|id| (id.0, id.1.parse().unwrap()))
        .collect();

    let step: Vec<&(usize, usize)> = pattern.iter()
        .filter(|&i| i.0 == 0)
        .collect();
    
    let step = step[0].1;

    let step = size + (step - (size % step));
    
    let mut x = 0;
    loop {
        let matches = pattern.iter()
            .filter(|&i| (x + i.0) % i.1 == 0)
            .count();
        
        if matches == pattern.len() {
            break;
        }

        x += step;
    }

    println!("{}", x);
}
