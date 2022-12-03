use std::fs;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Wrong file name!");
    let mut score: u32 = 0;
    for line in contents.split("\n") {
        if !line.is_empty() {
            let line_vec: Vec<char> = line.chars().collect();
            let mut pocket1: Vec<char> = line_vec[0..(line_vec.len()/2)].to_vec();
            let mut pocket2: Vec<char> = line_vec[(line_vec.len()/2)..].to_vec();
            pocket1.sort();
            pocket2.sort();
            pocket1.dedup();
            pocket2.dedup();
            let mut pockets: Vec<char> = pocket1;
            pockets.extend(pocket2);
            let mut frequency: HashMap<&char, u32> = HashMap::new();
            for char in &pockets {
                *frequency.entry(char).or_insert(0) += 1;
            }
            for (k, _v) in frequency.iter()
                    .filter(|&(_k, v)| (*v) > 1)
                    .map(|(k, v)| (k, v)) {
                let item: char = **k;
                if item.is_lowercase() {
                    score += ((item as u8) - 96) as u32;
                } else {
                    score += (((item as u8) - 38)) as u32;
                }
            }
            //println!("{:?}", frequency);        
        }
    }
    println!("score {}", score);
    let mut score2: u32 = 0;
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut groups: i32 = lines.len() as i32;
    groups = groups / 3;
    //println!("there are {} groups", groups);
    for group in 0..groups {
        let index:usize = (group * 3).try_into().unwrap();
        //println!("group {:?}", group);
        let mut pack1: Vec<char> = lines[index].chars().collect();
        let mut pack2: Vec<char> = lines[index+1].chars().collect();
        let mut pack3: Vec<char> = lines[index+2].chars().collect();
        pack1.sort();
        pack2.sort();
        pack3.sort();
        pack1.dedup();
        pack2.dedup();
        pack3.dedup();
        let mut packs: Vec<char> = pack1;
        packs.extend(pack2);
        packs.extend(pack3);
        let mut frequency: HashMap<&char, u32> = HashMap::new();
            for char in &packs {
                *frequency.entry(char).or_insert(0) += 1;
            }
            for (k, _v) in frequency.iter()
                    .filter(|&(_k, v)| (*v) == 3)
                    .map(|(k, v)| (k, v)) {
                let item: char = **k;
                if item.is_lowercase() {
                    score2 += ((item as u8) - 96) as u32;
                } else {
                    score2 += (((item as u8) - 38)) as u32;
                }
            }
    }
    println!("score2 {}", score2);
}
