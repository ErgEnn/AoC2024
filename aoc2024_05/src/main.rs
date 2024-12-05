use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    let lines = include_str!("../input.txt")
        .lines()
        .collect::<Vec<&str>>();

    first_problem(&lines);

    second_problem(&lines);
}

fn include_str(_: &str) -> &'static str {
    "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
}

fn first_problem(lines: &Vec<&str>){
    let mut rules = HashMap::<&str, Vec<&str>>::new();
    let mut valid_updates = Vec::<Vec<&str>>::new();
    for line in lines {
        if line.contains('|') {
            let ruleParts: Vec<&str> = line.split('|').collect();
            let before = ruleParts[0];
            let after = ruleParts[1];
            rules.entry(before).or_insert(Vec::new()).push(after);
        }
        if line.contains(',') {
            let update = line.split(',').collect::<Vec<&str>>();
            if(is_valid_update(&update, &rules)) {
                valid_updates.push(update);
            }
        }
    }

    let result: i32 = valid_updates.iter().map(|x| {x[x.len()/2].parse::<i32>().unwrap()}).sum();
    println!("Answer 1: {}", result);
}

fn is_valid_update(update: &Vec<&str>, rules: &HashMap<&str,Vec<&str>>) -> bool {
    for (updIdx, pageNoInUpdate) in update.iter().enumerate() {
        if rules.contains_key(pageNoInUpdate) {
            let pageRules = &rules[pageNoInUpdate];
            for pageRule in pageRules.iter() {
                match update.iter().position(|x| x == pageRule) {
                    None => {}
                    Some(idx) => {
                        if idx < updIdx {
                            return false;
                        }
                    }
                }
            }
        }
    }
    true
}

fn second_problem(lines: &Vec<&str>){
    let mut rules = HashMap::<&str, Vec<&str>>::new();
    let mut valid_updates = Vec::<Vec<&str>>::new();
    for line in lines {
        if line.contains('|') {
            let ruleParts: Vec<&str> = line.split('|').collect();
            let before = ruleParts[0];
            let after = ruleParts[1];
            rules.entry(before).or_insert(Vec::new()).push(after);
        }
        if line.contains(',') {
            let update = line.split(',').collect::<Vec<&str>>();
            if(!is_valid_update(&update, &rules)) {
                valid_updates.push(rewrite_update(update, &rules));
            }
        }
    }

    let result: i32 = valid_updates.iter().map(|x| {x[x.len()/2].parse::<i32>().unwrap()}).sum();
    println!("Answer 2: {}", result);
}

fn rewrite_update<'a>(mut update: Vec<&'a str>, rules: &HashMap<&str,Vec<&str>>) -> Vec<&'a str>{
    update.sort_by(|a, b| {
        if rules.get(a).map_or(false ,|r| r.contains(b)) {
            return Ordering::Less
        }
        if rules.get(b).map_or(false ,|r| r.contains(a)) {
            return Ordering::Greater
        }
        Ordering::Equal
    });

    update
}
