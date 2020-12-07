extern crate advent_of_code_2020;

use self::advent_of_code_2020::*;
use std::collections::HashMap;
use std::collections::HashSet;

type RuleSet = HashMap<String, Vec<(String, u32)>>;

fn main() {
    let input = get_input("day_7.txt");

    let rule_set: RuleSet = input
        .replace(" bags", "")
        .replace(" bag", "")
        .replace(".", "")
        .split("\n")
        .filter(|line| {
            line.matches("no other").count() == 0 && line.len() > 0
        })
        .map(|line| {
            let parts: Vec<&str> = line.split(" contain ").collect();
            let name = String::from(&parts[0][..]);
            
            let contains = parts[1].trim().split(", ").map(|bag| {
                let bag = bag.trim();
                (String::from(&bag[2..]), *&bag[..1].parse().unwrap())
            }).collect();

            (name, contains)
        }
    )
    .collect();

    println!("{}", find_parents_of_bag(&rule_set, "shiny gold").len());
    println!("{}", find_required_amount_of_bags(&rule_set, "shiny gold"));
}

fn find_parents_of_bag(rules: &RuleSet, bag: &str) -> HashSet<String> {
    let mut parents: HashSet<String> = rules.iter()
        .filter(|rule| {
            rule.1.iter().filter(|child| {
                child.0 == bag
            }).count() > 0
        })
        .map(|rule| {
            rule.0
        })
        .cloned()
        .collect();

    let check = parents.clone();

    if parents.len() > 0 {
        for parent in check {
            parents = parents.union(&find_parents_of_bag(rules, &parent[..])).cloned().collect();
        }
    }

    parents
}

fn find_required_amount_of_bags(rules: &RuleSet, bag: &str) -> u32 {
    let rule = rules.get(bag);
    match rule {
        None => 0,
        Some(rule) => {
            let mut amount: u32 = rule.iter()
                .map(|rule| {
                    rule.1
                })
                .sum();
    
            for bag in rule {
                amount += bag.1 * find_required_amount_of_bags(rules, &bag.0[..]);
            }

            amount
        }
    }
}
