use std::collections::HashMap;

fn main() {
    let start = vec!(9,12,1,4,17,0,18);
    let mut hm = HashMap::new();
    let mut pos = 0;
    let mut next_n = 0;
    for v in start {
        pos += 1;
        next_n = if let Some(last_pos) = hm.get(&v) {pos - *last_pos} else {0};
        hm.insert(v, pos);
    }
    while pos < 29_999_999 {
        //println!("{} - {} - {:?}", pos, next_n, hm);
        let n = next_n;
        pos += 1;
        next_n = if let Some(last_pos) = hm.get(&n) {pos - *last_pos} else {0};
        hm.insert(n, pos);
        if pos == 2019 {
            println!("2020 solution: {}", next_n);
        }
        //println!("{}: {}", pos, n);
    }
    println!("Result is {}", next_n);
}
