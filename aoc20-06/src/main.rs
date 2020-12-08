use std::fs;
use std::collections::HashSet;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let mut curr_group_any = String::new();
    let mut curr_group_all : HashSet<_> = ('a'..='z').collect();
    let mut result_any = 0;
    let mut result_all = 0;
    for l in contents.lines() {
        if l != "" {
            curr_group_any.push_str(l);
            let hs : HashSet<_> = l.chars().collect();
            curr_group_all = curr_group_all.intersection(&hs).cloned().collect();
        }
        else {
            for c in 'a'..='z' {
                if curr_group_any.contains(c) {result_any += 1};
            }
            result_all += curr_group_all.len();
            curr_group_any = "".to_string();
            curr_group_all = ('a'..='z').collect();
        }
    }
    println!("{} {}", result_any, result_all);
}
