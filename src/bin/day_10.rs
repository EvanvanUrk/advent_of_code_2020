extern crate advent_of_code_2020;

use self::advent_of_code_2020::*;

fn main() {
    let input = get_input("day_10_test_2.txt");

    let mut joltages: Vec<u32> = input.split('\n')
        .filter(|line| line.len() > 0)
        .map(|line| line.parse().unwrap())
        .collect();

    joltages.append(&mut vec![0u32]);
    joltages.sort();
    joltages.push(joltages.last().unwrap() + 3);

    let diffs: Vec<u32> = joltages.windows(2)
        .map(|diff| diff[1] - diff[0])
        .collect();

    let part_1 = diffs.iter().filter(|&x| *x == 1).count() * diffs.iter().filter(|&x| *x == 3).count();

    println!("{}", part_1);

    let mut contiguous_lens: Vec<u32> = Vec::new();
    let mut prev = joltages[0];
    let mut contiguous = 0;
    for j in &joltages[1..] {
        if *j == prev + 1 {
            contiguous += 1;
        } else if contiguous > 2 {
            contiguous_lens.push(contiguous);
            contiguous = 0;
        }
        prev = *j;
    }

    println!("{:?}", contiguous_lens);

    let part_2: Vec<u32> = contiguous_lens.iter()
        .map(|&x| 3u32.pow(x - 3) + 1)
        .collect();

    println!("{:?}", part_2);
    println!("{}", part_2.iter().product::<u32>());

    // joltages.reverse();

    // let combinations: usize = joltages.iter()
    //     .map(|&x| {
    //         joltages.iter()
    //             .filter(|&y| *y > x)
    //             .filter(|&y| *y - x <= 3)
    //             .count()
    //     })
    //     .filter(|&x| x > 1)
    //     .sum();

    // println!("{:?}", combinations);
}
