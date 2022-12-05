use std::fs;
use regex::Regex;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Wrong file name!");
    let mut columns: Vec<String> = [].to_vec();
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
// CrateMover 9000
    for line in contents.split("\n") {
        if !line.is_empty() {
            let line_vec: Vec<char> = line.chars().collect();
            let length = line_vec.len();
            let column_count = (length+1)/4;
            if line_vec.contains(&'[') {
                for column in 0..column_count {
                    if line_vec[(column*4)+1] != ' ' {
                        if column >= columns.len() {
                            columns.resize(column+1, "".to_string());
                        }
                        columns[column].push(line_vec[(column*4)+1]);
                    }
                }
            }
        } else {
            break;
        }
    }
    for cap in re.captures_iter(&contents) {
        let move_count: usize = cap[1].parse::<usize>().unwrap();
        let move_source: usize = cap[2].parse::<usize>().unwrap();
        let move_target: usize = cap[3].parse::<usize>().unwrap();
        let moved_containers = &columns[move_source-1][0..move_count];
        let moved_containers = &moved_containers.chars().rev().collect::<String>();
        if &columns[move_source-1].len() > &move_count {
            let remaining_containers = &columns[move_source-1][move_count..];
            columns[move_source-1] = remaining_containers.to_string();    
        } else {
            columns[move_source-1] = "".to_string();
        }
        columns[move_target-1] = moved_containers.to_owned() + &columns[move_target-1];
    }
    let mut answer: String = "".to_string();
    for column in 0..columns.len() {
        answer = answer + &columns[column][0..1];
    }
    println!("9000 answer is {}", answer);
// CrateMover 9001
    columns = [].to_vec();
    for line in contents.split("\n") {
        if !line.is_empty() {
            let line_vec: Vec<char> = line.chars().collect();
            let length = line_vec.len();
            let column_count = (length+1)/4;
            if line_vec.contains(&'[') {
                for column in 0..column_count {
                    if line_vec[(column*4)+1] != ' ' {
                        if column >= columns.len() {
                            columns.resize(column+1, "".to_string());
                        }
                        columns[column].push(line_vec[(column*4)+1]);
                    }
                }
            }
        } else {
            break;
        }
    }
    for cap in re.captures_iter(&contents) {
        let move_count: usize = cap[1].parse::<usize>().unwrap();
        let move_source: usize = cap[2].parse::<usize>().unwrap();
        let move_target: usize = cap[3].parse::<usize>().unwrap();
        let moved_containers = &columns[move_source-1][0..move_count];
        let moved_containers = &moved_containers.chars().collect::<String>();
        if &columns[move_source-1].len() > &move_count {
            let remaining_containers = &columns[move_source-1][move_count..];
            columns[move_source-1] = remaining_containers.to_string();    
        } else {
            columns[move_source-1] = "".to_string();
        }
        columns[move_target-1] = moved_containers.to_owned() + &columns[move_target-1];
    }
    let mut answer: String = "".to_string();
    for column in 0..columns.len() {
        answer = answer + &columns[column][0..1];
    }
    println!("9001 answer is {}", answer);
}

