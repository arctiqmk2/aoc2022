use std::fs;
use array_tool::vec::Intersect;
use regex::Regex;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Wrong file name!");
    let mut overlaps: u32 = 0;
    let mut partial_overlaps: u32 = 0;
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    for cap in re.captures_iter(&contents) {
        let range1start: i32 = cap[1].parse::<i32>().unwrap();
        let range1stop: i32 = cap[2].parse::<i32>().unwrap();
        let range2start: i32 = cap[3].parse::<i32>().unwrap();
        let range2stop: i32 = cap[4].parse::<i32>().unwrap();
        if range1start <= range2start && range1stop >= range2stop {
            overlaps += 1;
        } else if range2start <= range1start && range2stop >= range1stop {
            overlaps += 1;
        }
        let range1: Vec<i32> = (range1start..range1stop+1).collect();
        let range2: Vec<i32> = (range2start..range2stop+1).collect();
        let partial = range1.intersect(range2);
        if !partial.is_empty() {
            partial_overlaps += 1;
        }
    }
    println!("overlaps: {}", overlaps);
    println!("partial_overlaps: {}", partial_overlaps);
}
