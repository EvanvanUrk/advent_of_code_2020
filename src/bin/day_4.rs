extern crate advent_of_code_2020;

use self::advent_of_code_2020::*;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let input = get_input("day_4.txt");
    
    let mut passports: Vec<HashMap<String, String>> = Vec::new();
    for passport_input in input.split("\n\n") {
        let mut passport: HashMap<String, String> = HashMap::new();
        let passport_input = passport_input.replace('\n', " ");
        for field in passport_input.trim().split(' ') {
            let pair: Vec<&str> = field.split(':').collect();
            passport.insert(String::from(pair[0]), String::from(pair[1]));
        }
        passports.push(passport);
    }

    let required_fields: [String; 7] = [
        String::from("byr"),
        String::from("iyr"),
        String::from("eyr"),
        String::from("hgt"),
        String::from("hcl"),
        String::from("ecl"),
        String::from("pid"),
    ];

    let mut valid_passports = 0;
    for passport in passports.iter() {
        let mut is_valid = true;
        for required_field in &required_fields[..] {
            match passport.get(required_field) {
                Some(_) => (),
                None => is_valid = false,
            };
        }

        if !is_valid {
            continue;
        }

        if !validate_year(passport.get("byr").unwrap(), 1920, 2002) {
            continue;
        }

        if !validate_year(passport.get("iyr").unwrap(), 2010, 2020) {
            continue;
        }

        if !validate_year(passport.get("eyr").unwrap(), 2020, 2030) {
            continue;
        }

        if !validate_hgt(passport.get("hgt").unwrap()) {
            continue;
        }

        if !validate_hcl(passport.get("hcl").unwrap()) {
            continue;
        }

        if !validate_ecl(passport.get("ecl").unwrap()) {
            continue;
        }

        if !validate_pid(passport.get("pid").unwrap()) {
            continue;
        }

        valid_passports += 1;
    }

    println!("{}", valid_passports);
}

fn validate_year(value: &str, min:u32, max: u32) -> bool {
    match value.parse::<u32>() {
        Err(_) => false,
        Ok(year) => year >= min && year <= max
    }
}

fn validate_hgt(value: &str) -> bool {
    let regex = Regex::new(r"[0-9]+(cm|in)").unwrap();
    if !regex.is_match(value) {
        return false;
    }

    let len = value.len();
    let measure = &value[len - 2..];
    let height = *&value[..len - 2].parse::<u32>().unwrap();

    if measure == "cm" {
        height >= 150 && height <= 193
    } else if measure == "in" {
        height >= 59 && height <= 76
    } else {
        false
    }
}

fn validate_hcl(value: &str) -> bool {
    let regex = Regex::new(r"#[a-f0-9]{6}").unwrap();
    regex.is_match(value)
}

fn validate_ecl(value: &str) -> bool {
    let regex = Regex::new(r"(amb|blu|brn|gry|grn|hzl|oth)").unwrap();
    regex.is_match(value)
}

fn validate_pid(value: &str) -> bool {
    let regex = Regex::new(r"^[0-9]{9}$").unwrap();
    regex.is_match(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_year() {
        assert!(validate_year("2020", 2010, 2020));
        assert!(validate_year("2020", 2020, 2030));

        assert!(!validate_year("2020", 1920, 2002));
        assert!(!validate_year("2019", 2020, 2030));
        assert!(!validate_year("2031", 2020, 2030));
    }

    #[test]
    fn test_validate_hgt() {
        assert!(validate_hgt("150cm"));
        assert!(validate_hgt("193cm"));
        assert!(validate_hgt("59in"));
        assert!(validate_hgt("76in"));

        assert!(!validate_hgt("149cm"));
        assert!(!validate_hgt("194cm"));
        assert!(!validate_hgt("58in"));
        assert!(!validate_hgt("77in"));
    }

    #[test]
    fn test_validate_hcl() {
        assert!(validate_hcl("#abc123"));
        assert!(!validate_hcl("abc123"));
        assert!(!validate_hcl("#abz123"));
    }

    #[test]
    fn test_validate_ecl() {
        assert!(validate_ecl("brn"));
        assert!(!validate_ecl("wat"));
    }

    #[test]
    fn test_validate_pid() {
        assert!(validate_pid("000000001"));
        assert!(!validate_pid("0123456789"));
    }
}
