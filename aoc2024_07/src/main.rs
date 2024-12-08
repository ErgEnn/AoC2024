fn main() {
    let elements = include_str("../input.txt")
        .lines()
        .map(|line| line.split_whitespace().map(str::to_string).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();

    first_problem(&elements);

    second_problem(&elements);
}

fn include_str(_: &str) -> &'static str {
    "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
}

fn first_problem(elements: &Vec<Vec<String>>){
    let mut sum = 0;
    for line in elements{
        let (goal_str, _) = line[0].split_once(':').unwrap();
        let goal = goal_str.parse::<u64>().unwrap();
        let components = line[1..].iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
        let first = components[0];
        if calculate(first,goal, &components, 1, &format!("{first}")) {
            sum += goal;
        }
    }
    println!("Answer 1: {}", sum);
}

fn calculate(result_so_far: u64, goal: u64, components: &Vec<u64>, curr_idx: usize, debug: &str) -> bool{
    if curr_idx == components.len() {
        if result_so_far == goal {
            //print!("{}=", goal);
            return true;
        }
        //println!("{} over and end", debug);
        return false;
    }

    if result_so_far > goal {
        //println!("{} over", debug);
        return false;
    }

    let curr = components[curr_idx];
    let product = result_so_far * curr;
    //println!("{}*{} = {} ({})", debug, curr, product, goal);
    if calculate(product, goal, components, curr_idx+1, &format!("{debug}*{curr}")) {
        return true;
    }
    let sum = result_so_far + curr;
    //println!("{}+{} = {} ({})", debug, curr, sum, goal);
    if calculate(sum, goal, components, curr_idx+1,&format!("{debug}+{curr}")) {
        return true;
    }
    false
}

fn second_problem(elements: &Vec<Vec<String>>){
    let mut sum = 0;
    for line in elements{
        let (goal_str, _) = line[0].split_once(':').unwrap();
        let goal = goal_str.parse::<u64>().unwrap();
        let components = line[1..].iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
        let first = components[0];
        if calculate2(first,goal, &components, 1, &format!("{first}")) {
            sum += goal;
        }
    }
    println!("Answer 2: {}", sum);
}

fn calculate2(result_so_far: u64, goal: u64, components: &Vec<u64>, curr_idx: usize, debug: &str) -> bool{
    if curr_idx == components.len() {
        if result_so_far == goal {
            //print!("{}=", goal);
            return true;
        }
        //println!("{} over and end", debug);
        return false;
    }

    if result_so_far > goal {
        //println!("{} over", debug);
        return false;
    }

    let curr = components[curr_idx];
    let product = result_so_far * curr;
    //println!("{}*{} = {} ({})", debug, curr, product, goal);
    if calculate2(product, goal, components, curr_idx+1, &format!("{debug}*{curr}")) {
        return true;
    }
    let sum = result_so_far + curr;
    //println!("{}+{} = {} ({})", debug, curr, sum, goal);
    if calculate2(sum, goal, components, curr_idx+1, &format!("{debug}+{curr}")) {
        return true;
    }

    let concat = result_so_far * ((10u64).pow((curr as f64).log10().floor() as u32 + 1)) + curr;
    //println!("{}||{} = {} ({})", debug, curr, sum, goal);
    if calculate2(concat, goal, components, curr_idx+1, &format!("{debug}||{curr}")) {
        return true;
    }
    false
}
