use std::fs;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let mut nbs :Vec<_> = contents.lines().map(|s| s.parse::<i32>().unwrap()).collect();
    nbs.sort();
    let mut curr = 0;
    let mut s1 = 0;
    let mut s3 = 1;
    let mut hs = HashMap::new();
    hs.insert(0, 1);
    let mut result = 0i64;
    for n in nbs{
        if n == curr +1 {s1+=1;}
        else if n == curr +3 {s3+=1;}
        curr = n;
        result = 0;
        for i in 1..=3 {
            if let Some(v) = hs.get(&(n-i)) {
                result += v;
            }
        }
        hs.insert(n, result);
    }
    println!("{}*{}={}",s1,s3,s1*s3);
    println!("{}", result);
}
