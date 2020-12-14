use std::fs;
use std::collections::HashMap;

fn get_pos_mask(s: &str) -> u64{
    let s = s.replace("X", "0");
    u64::from_str_radix(&s, 2).unwrap()
}

fn get_neg_mask(s: &str) -> u64{
    let s = s.replace("X", "1");
    u64::from_str_radix(&s, 2).unwrap()
}

fn gen_pos(pos: u64, m: &str, start : usize) -> Vec<u64>
{
    let mut result = vec!();
    let ch = m.chars().nth(35-start).unwrap();
    let dd = match ch {
        '0' => vec![(pos>>start)&1],
        '1' => vec![1],
        'X' => vec![0,1],
        _ => panic!()
    };
    if start == 35 {
        result = dd;
    }
    else {
        let base = gen_pos(pos, m, start +1);
        for b in base {
            for d in &dd {
                result.push((b<<1)+d);
            }
        }
    }
    //println!("{} {} {} -> {:?}", pos, m, start, result);
    result
}

fn main() {
    //println!("{:?}", gen_pos(42, "000000000000000000000000000000X1001X", 0));
    //println!("{:?}", gen_pos(26, "00000000000000000000000000000000X0XX", 0));
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let mut pos_mask = 0;
    let mut neg_mask = 1<<36 - 1;
    let mut mask_str = "";
    let mut hm1 = HashMap::new();
    let mut hm2 = HashMap::new();
    for l in contents.lines() {
        let w : Vec<_>= l.split(" = ").collect();
        if w[0] == "mask" {
            mask_str = w[1];
            pos_mask = get_pos_mask(mask_str);
            neg_mask = get_neg_mask(mask_str);
            //println!("mask {:#36b} {:#36b}", pos_mask, neg_mask);
        }
        else {
            let pos = w[0][4..w[0].len()-1].parse::<u64>().unwrap();
            let val = w[1].parse::<u64>().unwrap();
            //println!("pos {}: {}", pos, val);
            hm1.insert(pos, (val | pos_mask) & neg_mask);
            let pos2 = gen_pos(pos, mask_str, 0);
            for p in pos2 {
                hm2.insert(p, val);
            }
        }
    }
    let result1 = hm1.iter().fold(0, |acc, (_k, v)| acc + v);
    let result2 = hm2.iter().fold(0, |acc, (_k, v)| acc + v);
    println!("Sum1 is {}", result1);
    println!("Sum2 is {}", result2);
}
