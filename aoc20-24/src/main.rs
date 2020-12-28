use std::fs;
use std::collections::HashMap;

fn add_or_insert(hm: &mut HashMap<(i32, i32), u32>, key: (i32, i32), val: u32){
    if let Some(v) = hm.get_mut(&key) {
        *v+= val;
    }
    else {
        hm.insert(key, val);
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let mut tiles = HashMap::new();
    for l in contents.lines() {
        let mut ch = l.chars();
        let mut x = 0;
        let mut y = 0;
        while let Some(c) = ch.next() {
            match c {
                'e' => x-=1,
                'w' => x+=1,
                'n' => { y-=1; if Some('e') == ch.next() {x-=1;}}
                's' => { y+=1; if Some('w') == ch.next() {x+=1;}}
                _ => panic!()
            }
        }
        add_or_insert(&mut tiles, (x,y), 1);
    }
    let result = tiles.iter().filter(|&(_, v)| 0 != v&1).count();
    println!("{} black sides up", result);
    let mut blacks = vec!();
    for (t , c) in &tiles {
        if 0 != c&1 {
            blacks.push(t.clone());
        }
    }
    for _d in 1..=100 {
        tiles.clear();
        for (x, y) in &blacks {
            add_or_insert(&mut tiles, (*x-1, *y), 1);
            add_or_insert(&mut tiles, (*x+1, *y), 1);
            add_or_insert(&mut tiles, (*x-1, *y-1), 1);
            add_or_insert(&mut tiles, (*x, *y-1), 1);
            add_or_insert(&mut tiles, (*x, *y+1), 1);
            add_or_insert(&mut tiles, (*x+1, *y+1), 1);
        }
        let mut new_blacks = vec!();
        for b in &blacks {
            if Some(&1) == tiles.get(b) {
                new_blacks.push(b.clone());
            }
        }
        for (t, c) in &tiles {
            if *c == 2 {
                new_blacks.push(t.clone());
            }
        }
        blacks = new_blacks;
        println!("After day {}: {} tiles are black.", _d, blacks.len());
    }
}
