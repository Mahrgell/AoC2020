use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let keys : Vec<_> = contents.lines().map(|s| s.parse::<u64>().unwrap()).collect();
    let mut ls = vec!(0,0);
    for i in 0..2 {
        let mut val = 1;
        let mut cnt = 0;
        while val != keys[i] {
            val = (val * 7) % 20201227;
            cnt += 1;
        }
        println!("LS {}: {}", i, cnt);
        ls[i] = cnt;
    }
    for i in 0..2 {
        let subnr = keys[i];
        let mut val = 1;
        for _ in 0..ls[1-i] {
            val = (val * subnr) % 20201227;
        }
        println!("{}", val);
    }
}
