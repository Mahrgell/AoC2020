use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let mut dir = 0i32;
    let mut pos1 = (0,0);
    let mut pos2 = (0,0);
    let mut wp = (1,10);
    for l in contents.lines() {
        let ch : Vec<_> = l.chars().collect();
        let mut step_sz = 0i32;
        for i in 1..ch.len()
        {
            step_sz = step_sz * 10 + (ch[i].to_digit(10).unwrap() as i32);
        }
        match ch[0] {
            'N' => {pos1.0 += step_sz; wp.0 += step_sz},
            'S' => {pos1.0 -= step_sz; wp.0 -= step_sz},
            'E' => {pos1.1 += step_sz; wp.1 += step_sz},
            'W' => {pos1.1 -= step_sz; wp.1 -= step_sz},
            'L' => {dir -= step_sz/90; for _ in 0..step_sz/90 {wp = (wp.1, -wp.0)}},
            'R' => {dir += step_sz/90; for _ in 0..step_sz/90 {wp = (-wp.1, wp.0)}},
            'F' => {
                while dir < 0 {dir += 4};
                match dir%4 {
                    0 => pos1.1 += step_sz,
                    1 => pos1.0 -= step_sz,
                    2 => pos1.1 -= step_sz,
                    3 => pos1.0 += step_sz,
                    _ => {println!("dir = {} -> {}", dir, dir%4); panic!()}
                }
                pos2.0 += step_sz*wp.0;
                pos2.1 += step_sz*wp.1;
            }
            _ => panic!()
        }
        //println!("{} -> {},{} - dir: {}", l, pos1.0, pos1.1, dir%4);
        println!("{} -> {},{} - wp: {:?}", l, pos2.0, pos2.1, wp);
    }
    println!("Final pos#1: {:?}, Dist: {}", pos1, pos1.0.abs()+pos1.1.abs());
    println!("Final pos#2: {:?}, Dist: {}", pos2, pos2.0.abs()+pos2.1.abs());
}
