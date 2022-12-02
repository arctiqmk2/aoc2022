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
        let mut line_score: i8 = 0;
        let mut second_line_score: i8 = 0;
        if !line.is_empty() {
            let line_vec: Vec<char> = line.chars().collect();
            let first: char = line_vec[0];
            let last: char = line_vec[line_vec.len()-1];
            let second_last_val: i8;
            let first_val:i8 = (first as i8)-64;
            let last_val:i8 = (last as i8)-87;
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
            }
           //println!("{} line score", line_score);
            if last_val == 1 {
            // loss
                if first_val == 1 {
                    second_last_val = 3;
                } else if first_val == 2 {
                    second_last_val = 1;
                } else {
                    second_last_val = 2;
                }
                //println!("loss {} {} {}", first_val, last_val, second_last_val);
                second_line_score += second_last_val;
            } else if last_val == 2 {
                // draw
                second_last_val = first_val;
                //println!("draw {} {} {}", first_val, last_val, second_last_val);
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
                //println!("win {} {} {}", first_val, last_val, second_last_val);
                second_line_score += 6;
                second_line_score += second_last_val;
            }
            //println!("{} second line score", second_line_score);
        }
        score += line_score as i32;
        second_score += second_line_score as i32;
    }
    println!("total score {}", score);
    println!("second score {}", second_score);
}