use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let mut bags = HashMap::new();
    for l in contents.lines() {
        let substr : Vec<_> = l.split(" bags contain ").collect();
        //println!("{:?}", substr);
        let mut contained = vec!();
        if substr[1] != "no other bags." {
            let rhs_subs = substr[1].split(", ");
            for sub in rhs_subs {
                let sub_parts : Vec<_> = sub.split(' ').collect();
                //println!("{:?}", sub_parts);
                let nb = sub_parts[0].parse::<u32>().unwrap();
                let mut color = String::from(sub_parts[1]);
                color.push(' ');
                color.push_str(sub_parts[2]);
                contained.push((nb, color));
            }
        }
        bags.insert(String::from(substr[0]), contained);
    }
    let mut to_process = HashSet::new();
    to_process.insert(String::from("shiny gold"));
    let mut processed = HashSet::new();
    while !to_process.is_empty() {
        let mut next_to_process = HashSet::new();
        for s in &to_process {
            for (k, contained) in &bags{
                for (_, color) in contained {
                    if s == color && !processed.contains(k) && !to_process.contains(k) {
                        next_to_process.insert(k.clone());
                    }
                }
            }
            
            processed.insert(s.clone());
        }
        to_process = next_to_process;
        println!("{} processed, {} to process.", processed.len() - 1, to_process.len());
        //println!("{:?}", processed);
    }

    let mut to_process = vec!();
    to_process.push((1, String::from("shiny gold")));
    let mut result = 0;
    while !to_process.is_empty() {
        let mut next_to_process = vec!();
        for (nb_s, s) in &to_process {
            for (nb_c, c) in &bags[s]{
                next_to_process.push((nb_s*nb_c, c.clone()));
            }
            result += nb_s;
        }
        to_process = next_to_process;
        println!("{} bags found, {} entries to process.", result - 1, to_process.len());
        println!("{:?}", to_process);

    }
}