use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    //println!("{}",contents);
    let numbers: Vec<_> = contents.lines().map(|l| l.parse::<i32>().unwrap()).collect();
    println!("{:?}", numbers);
    let mut exists = [false; 2020];
    for i in numbers.iter() {
        let ii = *i as usize;
        let req_value = (2020 - i) as usize;
        if exists[req_value]{
            println!("Solution found: {}", ii * req_value);
            break;
        }
        else {
            exists[ii] = true;
        }
    }
    let mut sorted = numbers;
    sorted.sort();
    let mut i1 = 0;
    while i1 < sorted.len() {
        let v1 = sorted[i1];
        let mut i2 = i1 + 1;
        while i2 < sorted.len() {
            let v2 = sorted[i2];
            if v1+v2 > 2020 {
                break;
            }
            if sorted.contains(&(2020-v1-v2)) {
                println!("Second solution: {}", v1*v2*(2020-v1-v2));
                i1 = 99999;
                break;
            }
            i2 += 1;
        }

        i1 += 1;
    }

}
