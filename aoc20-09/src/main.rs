use std::fs;
use itertools::Itertools;
use std::cmp::Ordering;
use std::cmp;

fn is_valid(preamble: &[u64], val: u64) -> bool
{
    for (a,b) in preamble.iter().tuple_combinations() {
        if a + b == val {return true;}
    }
    return false;
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let values : Vec<_>= contents.lines().map(|s| s.parse::<u64>().unwrap()).collect();
    let mut result1 = 0;
    for i in 25..values.len() {
        if !is_valid(&values[i-25 .. i], values[i]) {
            result1 = values[i];
            break;
        }
    }
    println!("{} is the first wrong number.", result1);
    let mut result2 = 0;
    for first_pos in 0..values.len() {
        let mut sum = 0;
        let mut min = values[first_pos];
        let mut max = values[first_pos];
        for i in first_pos..values.len() {
            sum += values[i];
            min = cmp::min(min, values[i]);
            max = cmp::max(max, values[i]);
            match sum.cmp(&result1) {
                Ordering::Equal => {
                    result2 = min+max; break;},
                Ordering::Greater => break,
                _ => (),
            }
        }
        if result2 != 0 {break}
    }
    println!("{} is the second solution.", result2);
}
