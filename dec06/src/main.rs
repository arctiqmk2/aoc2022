use std::fs;
use std::cmp;
use std::collections::HashMap;
use substring::Substring;


fn main() {
    let contents = fs::read_to_string("input.txt").expect("Wrong file name!");
    for line in contents.split("\n") {
        if !line.is_empty() {
            let (unique_string, position) = find_first_unique_set(4,line.to_string());
            println!("'{}' start-of-packet marker at pos {}", unique_string, position+4);
            let (unique_string, position) = find_first_unique_set(14,line.to_string());
            println!("'{}' start-of-message market at pos {}", unique_string, position+14);
        }
    }
}

pub fn get_longest_substring(s: String) -> (String, usize, usize) {
    let mut length = 0;
    let mut char_set: HashMap<char, usize> = HashMap::new();
    let mut start = 0;
    for (end, c) in s.char_indices() {
        if let Some(&n) = char_set.get(&c) {
            start = cmp::max(start, n);
        }
        length = cmp::max(length, end - start + 1);
        char_set.insert(c, end + 1);
    }
    (s.substring(start,start+length-1).to_string(), start, length-1)
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
    (uniq, start)
}