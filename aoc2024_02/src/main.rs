fn main() {
    let reports = include_str!("../input.txt")
        .lines()
        .map(|line| line.split_whitespace().map(str::to_string).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();

    first_problem(&reports);

    second_problem(&reports);
}

fn first_problem(reports: &Vec<Vec<String>>){
    let result = reports.iter().filter(|report| is_report_valid(report)).count();
    println!("Answer 1: {}", result);
}

fn is_report_valid(report: &Vec<String>)-> bool{
    let mut report_iter = report.iter();
    let mut prev = report_iter.next().unwrap().parse::<i32>().unwrap();
    let mut dir = 0;
    for reading_raw in report_iter {
        let reading = reading_raw.parse::<i32>().unwrap();
        if reading > prev && dir == -1 || reading < prev  && dir == 1{
            return false;
        }
        let diff = reading-prev;
        let diff_abs = diff.abs();
        if diff_abs <= 3 && diff_abs >= 1{
            dir = if reading > prev { 1 } else {-1};
            prev = reading;
            continue;
        }
        return false;
    }
    true
}

fn second_problem(reports: &Vec<Vec<String>>){
    let result = reports
        .iter()
        .filter(|report| {
        is_report_valid(report) ||
            report
                .iter()
                .enumerate()
                .map(|x| {[&report[..x.0], &report[x.0+1..]].concat()})
                .any(|x| is_report_valid(&x))
    })
        .count();
    println!("Answer 2: {}", result);
}


