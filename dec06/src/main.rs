use std::fs;
use substring::Substring;


fn main() {
    let contents = fs::read_to_string("input.txt").expect("Wrong file name!");
    for line in contents.split("\n") {
        if !line.is_empty() {
            let (unique_string, position) = find_first_unique_set(4,line.to_string());
            println!("'{}' start-of-packet marker at pos {}", unique_string, position+4);
            let (unique_string, position) = find_first_unique_set(14,line.to_string());
            println!("'{}' start-of-message marker at pos {}", unique_string, position+14);
        }
    }
}

pub fn find_first_unique_set(ul: usize, s: String) -> (String, usize) {
    let characters: Vec<char> = s.chars().collect();
    let mut uniq: String = "".to_string();
    let mut start: usize = 0;
    for i in 0..characters.len() {
        if !uniq.contains(characters[i]) {
            uniq.push(characters[i]);
        } else {
            if let Some(left) = uniq.find(characters[i]) {
                start = start + left + 1;
                uniq = uniq.substring(left+1,uniq.len()).to_string();
                uniq.push(characters[i]);
            }
        }
        let size = uniq.len();
        if size == ul { break };
    }
    if uniq.len() < ul {
        return ("".to_string(),0);
    } else {
        return (uniq, start);
    }
}