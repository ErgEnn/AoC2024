use std::time::Instant;

fn main() {
    let input = include_str!("../input.txt")
        .lines().into_iter().next().unwrap();

    first_problem(input);

    second_problem(input);
}

fn include_str(_: &str) -> &'static str {
    "2333133121414131402"
}

fn first_problem(input: &str){
    let mut files: Vec<u8> = Vec::new();
    let mut emptys: Vec<u8> = Vec::new();

    let mut is_file = true;
    for chr in input.chars() {
        if is_file {
            files.push(chr.to_digit(10).unwrap() as u8);
        }else{
            emptys.push(chr.to_digit(10).unwrap() as u8);
        }
        is_file = !is_file;
    }

    let mut disc: Vec<(u32,u8)> = Vec::new();
    let mut emptys_iter = emptys.iter();
    let mut file_idx: i64 = -1;
    is_file = true;
    loop {
        if is_file {
            file_idx += 1;
            if(file_idx >= files.len() as i64){
                break;
            }
            disc.push((file_idx as u32, files[file_idx as usize]));
        }else{
            match emptys_iter.next(){
                Some(x) => {
                    let mut empty_size = *x;
                    while empty_size > 0 {
                        if file_idx >= (files.len() as i64 - 1) {
                            break;
                        }
                        let file_id = files.len()-1;
                        let file_len = files.pop().unwrap();
                        let remaining_empty = empty_size as i8 - file_len as i8;
                        if remaining_empty < 0{
                            disc.push((file_id as u32, empty_size));
                            files.push( -remaining_empty as u8);
                            break;
                        }
                        if remaining_empty >= 0{
                            disc.push((file_id as u32, file_len));
                            empty_size = remaining_empty as u8;
                        }
                    }
                }
                None => {
                }
            }
        }
        is_file = !is_file;
    }

    //println!("{:?}", disc);

    let mut result: u64 = 0;
    let mut idx: usize = 0;
    for (file_id, file_len) in disc {
        for idx in idx..(idx+file_len as usize) {
            let product = (idx * file_id as usize) as u64;
            //println!("{idx}*{file_id}={product}");
            result += product;
        }
        idx += file_len as usize;
    }

    println!("Answer 1: {}", result);
}


fn print(disc: &Vec<(i32,u8)>){
    for (file_id, len) in disc.iter() {
        for _ in 0..*len {
            if *file_id == -1 {
                print!(".")
            }else{
                print!("{file_id}")
            }

        }
    }
    println!();
}
fn second_problem(input: &str){
    let now = Instant::now();
    let mut disc: Vec<(i32,u8)> = Vec::new();
    let mut is_file = true;
    for (idx, chr) in input.chars().enumerate() {
        let len = chr.to_digit(10).unwrap() as u8;
        if len != 0 {
            if is_file {
                disc.push(((idx/2) as i32, len));
            }else{
                disc.push((-1, len));
            }
        }
        is_file = !is_file;
    }

    disc.reverse();

    let mut file_idx: usize = 0;
    //println!("{:?} {}", disc, disc.len());
    //print(&disc);
    loop {
        if file_idx >= disc.len() {
            break;
        }
        let (file_id, file_size) = disc[file_idx];
        if file_id == -1 {
            file_idx += 1;
            continue;
        }
        //print!("{file_id}  ");
        for empty_idx in (file_idx+1..disc.len()).rev() {
            let (test, empty_size) = disc[empty_idx];
            if test != -1 {
                continue;
            }
            if file_size < empty_size {
                disc[empty_idx] = (file_id, file_size);
                disc[file_idx] = (-1, file_size);
                disc.insert(empty_idx, (test, empty_size-file_size));
                break;
            }
            if empty_size == file_size {
                disc[empty_idx] = (file_id, file_size);
                disc[file_idx] = (-1, file_size);
                break;
            }
        }
        //print(&disc);
        file_idx += 1;
    }

    disc.reverse();



    //print(&disc);
    let mut result: u64 = 0;
    let mut idx: usize = 0;
    for (file_id, file_len) in disc.iter().filter(|(_,len)| *len > 0) {
        let file_len = *file_len as usize;
        let file_id = *file_id as i64;
        if file_id != -1 {
            for idx in idx..(idx+file_len) {
                let product = (idx as i64 * file_id) as u64;
                //println!("{idx}*{file_id}={product}   {result}+{product}={}", result+product);
                result += product;
            }
        }
        idx += file_len;
    }

    println!("Answer 2: {} ({:?})", result, now.elapsed());
}
