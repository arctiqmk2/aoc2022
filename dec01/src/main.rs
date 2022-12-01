use std::fs;

fn main() {
    let contents = fs::read_to_string("/Users/mkennedy/Library/Mobile Documents/com~apple~CloudDocs/Projects/aoc2022/dec01/input.txt").expect("Wrong file name!");
    let mut elf: usize = 0;
    let mut elves: Vec<u32> = Vec::new();
    elves.push(0);
    for line in contents.split("\n") {
        if !line.is_empty() {
            let calories: u32 = line.parse::<u32>().unwrap();
            elves[elf] += calories;
            //println!("{:?}", calories);
        } else {
            //println!("{} calories for elf {}", elves[elf], elf);
            elf += 1;
            elves.push(0);
        }
    }
    elves.sort();
    let most_calories = elves[elves.len()-1];
    let top_three_total = elves[elves.len()-1] + elves[elves.len()-2] + elves[elves.len()-3];
    println!("most calories is {}", most_calories);
    println!("top three total is {}", top_three_total);
}