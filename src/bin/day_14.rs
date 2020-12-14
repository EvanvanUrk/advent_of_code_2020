extern crate advent_of_code_2020;

use self::advent_of_code_2020::*;
use itertools::Itertools;
use std::collections::HashMap;

type Mask = HashMap<usize, Option<bool>>;
type Values = HashMap<usize, u64>;

fn main() {
    let input = get_input("day_14.txt");
    let lines = input.split('\n').filter(|&l| l.len() > 0);

    let mut mask: Mask = HashMap::new();
    let mut values: Values = HashMap::new();

    for line in lines.clone() {
        let parts: Vec<&str> = line.split(" = ").collect();
        let instruction = parts[0];
        let value = parts[1];

        if &instruction[0..3] == "mem" {
            let addr: usize = instruction[4..instruction.len() - 1].parse().unwrap();
            let mut bvec = dtobvec(value.parse().unwrap(), 36);
            for pair in mask.iter() {
                if let Some(bit) = pair.1 {
                    bvec[*pair.0] = *bit;
                }
            }
            let value = bvectod(bvec);
            *values.entry(addr).or_insert(value) = value;
        } else if instruction == "mask" {
            mask = parse_mask(value);
        }
    }

    let sum: u64 = values.values().sum();

    println!("{}", sum);

    let mut mask: Mask = HashMap::new();
    let mut values: Values = HashMap::new();

    for line in lines {
        let parts: Vec<&str> = line.split(" = ").collect();
        let instruction = parts[0];
        let value = parts[1];

        if &instruction[0..3] == "mem" {
            let addr: u64 = instruction[4..instruction.len() - 1].parse().unwrap();
            let addr_space: Vec<Option<bool>> = dtobvec(addr, 36).iter()
                .zip(mask.iter().sorted_by_key(|&x| x.0))
                .map(|i| {
                    if let Some(bit) = i.1.1 {
                        Some(*i.0 || *bit)
                    } else {
                        None
                    }
                })
                .collect();

            let floating_count = addr_space.iter().filter(|&i| *i == None).count();
            for mut i in 0..2u32.pow(floating_count as u32) {
                let mut j = floating_count;
                let mut addr = addr_space.clone();
                while j > 0 {
                    let bit = i % 2 == 1;
                    let first_floating = addr.iter().position(|&elem| elem == None).unwrap();
                    addr[first_floating] = Some(bit);
                    i = i >> 1;
                    j -= 1;
                }

                let addr = bvectod(
                    addr.iter()
                        .map(|&i| {
                            match i {
                                Some(bit) => bit,
                                None => unreachable!(),
                            }
                        })
                        .collect()
                );

                let value = value.parse().unwrap();
                *values.entry(addr as usize).or_insert(value) = value;
            }
        } else if instruction == "mask" {
            mask = parse_mask(value);
        }
    }

    let sum: u64 = values.values().sum();

    println!("{}", sum);
}

fn parse_mask(value: &str) -> Mask {
    value.chars()
        .enumerate()
        .map(|l| {
            match l.1 {
                '0' => (l.0, Some(false)),
                '1' => (l.0, Some(true)),
                'X' => (l.0, None),
                _ => unreachable!(),
            }
        })
        .collect()
}

fn dtobvec(mut d: u64, len: usize) -> Vec<bool> {
    let mut vec = Vec::new();
    while d != 0 {
        vec.push(d % 2 == 1);
        d = d >> 1;
    }

    vec.iter().pad_using(len, |_| &false).cloned().rev().collect()
}

fn bvectod(vec: Vec<bool>) -> u64 {
    u64::from_str_radix(
        &vec.iter()
            .map(|&x| if x { '1' } else { '0' })
            .join("")[..],
        2
    ).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dtobvec() {
        assert_eq!(
            vec![true, false, true],
            dtobvec(5, 3)
        );

        assert_eq!(
            vec![true, true, true],
            dtobvec(7, 3)
        );

        assert_eq!(
            vec![false, false, false, false, false, true, false, true],
            dtobvec(5, 8)
        );
    }

    #[test]
    fn test_bvectod() {
        assert_eq!(
            5,
            bvectod(vec![true, false, true])
        );

        assert_eq!(
            7,
            bvectod(vec![true, true, true])
        );

        assert_eq!(
            5,
            bvectod(vec![false, false, false, false, false, true, false, true])
        );
    }
}
