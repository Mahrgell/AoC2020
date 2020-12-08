use std::fs;

extern crate regex;

use regex::Regex;

fn main() {
    let mut counter1 = 0;
    let mut counter2 = 0;
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let re = Regex::new(r"(\d+)-(\d+) ([a-z]{1}): ([a-z]+)").unwrap();
    for l in contents.lines(){
        for cap in re.captures_iter(l) {
            println!("{} {} {} {}", &cap[1], &cap[2], &cap[3], &cap[4]);
            let c = cap[4].matches(&cap[3]).count();
            let min = cap[1].parse::<usize>().unwrap();
            let max = cap[2].parse::<usize>().unwrap();
            if (min <= c) && (max >= c) {
                counter1 += 1;
            }
            let ch1 = (&cap[4]).chars().nth(min-1).unwrap();
            //println!("{}: {} = {}", min, ch1, &cap[3]);
            let ch2 = (&cap[4]).chars().nth(max-1).unwrap();
            //println!("{}: {} {}: {} = {}", min, ch1, max, ch2, &cap[3]);
            let cf = (&cap[3]).chars().nth(0).unwrap();
            if (ch1 == cf) != (ch2 == cf){
                counter2 += 1;
            }
        }
    }
    println!("{} legal passwords found by rule 1!", counter1);
    println!("{} legal passwords found by rule 2!", counter2);
}

