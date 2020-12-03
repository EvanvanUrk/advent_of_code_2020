extern crate advent_of_code_2020;

use self::advent_of_code_2020::*;
use regex::Regex;

fn main() {
    let input = get_input("day_2.txt");

    let lines = input.matches("\n").count();
    let mut passwords: Vec<Password> = Vec::with_capacity(lines);

    let regex = Regex::new(r"^(\d+)-(\d+) ([a-zA-Z]): ([a-zA-Z]+)$").unwrap();

    for line in input.split("\n") {
        match regex.captures(line) {
            None => (),
            Some(caps) => {
                passwords.push(Password {
                    min_required: caps.get(1).unwrap().as_str().parse().expect("Could not parse input"),
                    max_required: caps.get(2).unwrap().as_str().parse().expect("Could not parse input"),
                    required_char: caps.get(3).unwrap().as_str().parse().expect("Could not parse input"),
                    password: String::from(caps.get(4).unwrap().as_str()),
                });
            }
        }
    }

    let valid_passwords = passwords.iter().filter(|password| {
        password.is_valid()
    });

    println!("Input has {} valid passwords out of {} lines", valid_passwords.count(), lines);

    let valid_passwords_2 = passwords.iter().filter(|password| {
        password.is_valid_new()
    });

    println!("{} passwords are valid by the new policy", valid_passwords_2.count());
}

struct Password {
    min_required: usize,
    max_required: usize,
    required_char: char,
    password: String
}

impl Password {
    fn is_valid(&self) -> bool {
        let count = self.password.matches(self.required_char).count();
        count >= self.min_required && count <= self.max_required
    }

    fn is_valid_new(&self) -> bool {
        self.password.match_indices(self.required_char).filter(|occurrence| {
            occurrence.0 + 1 == self.min_required || occurrence.0 + 1 == self.max_required
        }).count() == 1
    }
}
