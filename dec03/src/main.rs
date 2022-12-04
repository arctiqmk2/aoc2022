extern crate array_tool;
use std::fs;
use array_tool::vec::Intersect;

//score 7737
//badge priority 2697

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Wrong file name!");
    let mut score: u32 = 0;
    for line in contents.split("\n") {
        if !line.is_empty() {
            let line_vec: Vec<char> = line.chars().collect();
            let pocket1: Vec<char> = line_vec[0..(line_vec.len()/2)].to_vec();
            let pocket2: Vec<char> = line_vec[(line_vec.len()/2)..].to_vec();
            let bothpockets: Vec<char> = pocket1.intersect(pocket2);
            for item in &bothpockets {
                if item.is_lowercase() {
                    score += ((*item as u8) - 96) as u32;
                } else {
                    score += (((*item as u8) - 38)) as u32;
                }
            }
        }
    }
    println!("score {}", score);
    // part 2
    let mut score2: u32 = 0;
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut groups: i32 = lines.len() as i32;
    groups = groups / 3;
    for group in 0..groups {
        let index:usize = (group * 3).try_into().unwrap();
        let pack1: Vec<char> = lines[index].chars().collect();
        let pack2: Vec<char> = lines[index+1].chars().collect();
        let pack3: Vec<char> = lines[index+2].chars().collect();
        let first_two_packs: Vec<char> = pack1.intersect(pack2);
        let all_packs: Vec<char> = first_two_packs.intersect(pack3);
        for item in &all_packs {
            if item.is_lowercase() {
                score2 += ((*item as u8) - 96) as u32;
            } else {
                score2 += (((*item as u8) - 38)) as u32;
            }
        }
    }
    println!("badge priority {}", score2);
}
