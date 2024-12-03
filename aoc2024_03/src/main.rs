use regex::Regex;
fn main() {
    let input = include_str!("../input.txt");

    first_problem(&input);

    second_problem(&input);
}

fn first_problem(input: &str){
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let result = re.captures_iter(input).map(|cap| {
        let a = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let b = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let c = a*b;
        c
    }).sum::<i32>();
    println!("Answer 1: {}", result);
}

fn second_problem(input: &str){
    let re = Regex::new(r"(mul|don't|do)(?:\((?:|(\d+),(\d+))\))").unwrap();
    let mut enabled = true;
    let result = re.captures_iter(input).map(|cap| {
        match cap.get(1).unwrap().as_str() {
            "mul" => if enabled {cap.get(2).unwrap().as_str().parse::<i32>().unwrap() * cap.get(3).unwrap().as_str().parse::<i32>().unwrap()} else {0},
            "don't" => {enabled = false; 0}
            "do" => {enabled = true; 0}
            _ => 0
        }
    }).sum::<i32>();
    println!("Answer 2: {}", result);
}
