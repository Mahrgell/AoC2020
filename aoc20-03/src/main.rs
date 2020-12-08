use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let mut trees = Vec::new();
    let maxoffset = 31;
    for l in contents.lines() {
        let mut val: u32 = 0;
        let mut offset = 0;
        for c in l.chars() {
            if c == '#' {
                val |= 1<<offset;
            }
            offset += 1;
        }
        trees.push(val);
    }
    let mut treehits11 : u64 = 0;
    let mut treehits31 : u64 = 0;
    let mut treehits51 : u64 = 0;
    let mut treehits71 : u64 = 0;
    let mut treehits12 : u64 = 0;
    for i in 0..trees.len()
    {
        if 0 != (trees[i] & (1<<((1*i)%maxoffset))){
            println!("hit!");
            treehits11 += 1;
        }
        if 0 != (trees[i] & (1<<((3*i)%maxoffset))){
            println!("hit!");
            treehits31 += 1;
        }
        if 0 != (trees[i] & (1<<((5*i)%maxoffset))){
            println!("hit!");
            treehits51 += 1;
        }
        if 0 != (trees[i] & (1<<((7*i)%maxoffset))){
            println!("hit!");
            treehits71 += 1;
        }
        if i%2 == 0 && (0 != (trees[i] & (1<<((i/2)%maxoffset)))){
            println!("hit!");
            treehits12 += 1;
        }
    }
    println!("{} trees(31) hit.", treehits31);
    println!("Total tree factor {}.", treehits11*treehits31*treehits51*treehits71*treehits12);
}
