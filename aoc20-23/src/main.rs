fn game_brute(pos: &mut Vec<u32>, rounds: usize){
    for _i in 1..=rounds {
        let active = pos[0];
        let mut moved = vec!(pos[1], pos[2], pos[3]);
        let mut dest = if active == 1 {9} else {active - 1};
        while moved.contains(&dest) {
            dest = if dest == 1 {9} else {dest - 1};
        }
        let dest_idx = pos.iter().position(|x| *x == dest).unwrap();
        let mut new_pos = Vec::with_capacity(pos.len());
        new_pos.extend_from_slice(&pos[4..=dest_idx]);
        new_pos.append(&mut moved);
        if dest_idx != pos.len()-1 {
            new_pos.extend_from_slice(&pos[dest_idx+1..]);
        }
        new_pos.push(active);
        *pos = new_pos;
        //println!("{}: {:?}", _i, pos);
    }
}

fn game_smart(pos: &Vec<u32>, rounds: usize){
    let max_n = pos.len();
    let mut posa = vec![0 as usize; max_n+1];
    let mut prev = pos[max_n-1];
    for p in pos.iter() {
        posa[prev as usize] = *p as usize;
        prev = *p;
    }
    let mut active = pos[0] as usize;
    for _i in 1..=rounds {
        let mut moved = vec!();
        let mut n = active;
        for _ in 0..3 {
            n= posa[n];
            moved.push(n);
        }
        //println!("{:?}", moved);
        posa[active] = posa[moved[2]];
        let mut dest = if active == 1 {max_n} else {active - 1};
        while moved.contains(&dest) {
            dest = if dest == 1 {max_n} else {dest - 1};
        }
        posa[moved[2]] = posa[dest];
        posa[dest] = moved[0];
        active = posa[active];
        //println!("{:?}", &posa[..20]);
    }
    let first = posa[1];
    let second = posa[first];
    println!("{}", first*second);
}



fn main() {
    let mut pos = vec!(1,6,7,2,4,8,3,5,9);
    //let mut pos = vec!(3,8,9,1,2,5,4,6,7);
    let mut pos1 = pos.clone();
    game_brute(&mut pos1, 100);
    println!("{:?}",pos1);

    for i in 10..=1_000_000 {
        pos.push(i);
    }
    game_smart(&pos, 10_000_000);
}
