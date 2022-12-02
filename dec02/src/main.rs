use std::fs;
// A for Rock, B for Paper, and C for Scissors
// X for Rock, Y for Paper, and Z for Scissors
// Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock
// The winner of the whole tournament is the player with the highest score.
// Your total score is the sum of your scores for each round.
// The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors)
// plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).

fn main() {
    let contents = fs::read_to_string("/Users/mkennedy/Library/Mobile Documents/com~apple~CloudDocs/Projects/aoc2022/dec02/input.txt").expect("Wrong file name!");
    let mut score: i32 = 0;
    let mut second_score: i32 = 0;
    for line in contents.split("\n") {
        let mut line_score: i32 = 0;
        let mut second_line_score: i32 = 0;
        if !line.is_empty() {
            let line_vec: Vec<char> = line.chars().collect();
            let first: char = line_vec[0];
            let last: char = line_vec[line_vec.len()-1];
            let mut first_val: i32 = 0;
            let mut last_val: i32 = 0;
            let mut second_last_val: i32 = 0;
            if first == 'A' {
                first_val = 1;
            } else if first == 'B' {
                first_val = 2;
            } else if first == 'C' {
                first_val = 3;
            }
            if last == 'X' {
                last_val = 1;
            } else if last == 'Y' {
                last_val = 2;
            } else if last == 'Z' {
                last_val = 3;
            }
            println!("{} {}", first, last);
            println!("{} {}", first_val, last_val);
            line_score += last_val;
            if (last_val - first_val) == 0 {
                line_score += 3; 
            } else if (last_val - first_val) == 1 {
                line_score += 6;
            } else if (last_val - first_val) == 2 {
                line_score += 0;
            } else if (last_val - first_val) == -1 {
                line_score += 0;
            } else if (last_val - first_val) == -2 {
                line_score += 6;
            } else {
                println!{"unexpected."}
            }
            println!("{} line score", line_score);
            if last_val == 1 {
            // loss
            if first_val == 1 {
                second_last_val = 3;
            } else if first_val == 2 {
                second_last_val = 1;
            } else {
                second_last_val = 2;
            }
            println!("loss {} {} {}", first_val, last_val, second_last_val);
            second_line_score += second_last_val;
            } else if last_val == 2 {
            // draw
            second_last_val = first_val;
            println!("draw {} {} {}", first_val, last_val, second_last_val);
            second_line_score += 3;
            second_line_score += second_last_val;      
            } else if last_val == 3 {
            //win
            if first_val == 1 {
                second_last_val = 2;
            } else if first_val == 2 {
                second_last_val = 3;
            } else {
                second_last_val = 1;
            }
            println!("win {} {} {}", first_val, last_val, second_last_val);
            second_line_score += 6;
            second_line_score += second_last_val;
            }
            println!("{} second line score", second_line_score);
            } else {
        }
        score += line_score;
        second_score += second_line_score;
    }
    println!("total score {}", score);
    println!("second score {}", second_score);
}