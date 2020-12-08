use std::fs;
use std::collections::HashSet;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let mut max_seat = 0;
    let mut min_seat = 999;
    let mut seats = HashSet::new();
    for l in contents.lines() {
        let l = l.replace("B", "1");
        let l = l.replace("F", "0");
        let l = l.replace("R", "1");
        let l = l.replace("L", "0");
        let seat = u32::from_str_radix(&l, 2).unwrap();
        max_seat = std::cmp::max(max_seat, seat);
        min_seat = std::cmp::min(min_seat, seat);
        seats.insert(seat);
    }
    println!("max seat nb is {}.", max_seat);
    for i in (min_seat+1)..max_seat{
        if !seats.contains(&i){
            println!("seat {} is empty.", i);
            break;
        }
    }
}
