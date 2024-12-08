fn main() {
    let elements = include_str!("../input.txt")
        .lines()
        .map(|line| line.split_whitespace().map(str::to_string).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();

    first_problem(&elements);

    second_problem(&elements);
}

fn first_problem(elements: &Vec<Vec<String>>){
    let result = 0;
    println!("Answer 1: {}", result);
}

fn second_problem(elements: &Vec<Vec<String>>){
    let result = 0;
    println!("Answer 2: {}", result);
}
