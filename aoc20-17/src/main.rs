use std::fs;

fn inb(x: i32, max: i32) -> bool {
    x >= 0 && x<max
}

fn sol1() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let mut space = [[[false; 20]; 20]; 7];
    for (li, l) in contents.lines().enumerate() {
        for (ci, c) in l.chars().enumerate() {
            if c =='#' {
                space[0][6+li][6+ci] = true;
            }
        }
    }
    // for i in 0..20 {
    //     println!("{:?}", space[0][i]);
    // }
    for r in 0..6 {
        let mut new_space = space.clone();
        for z in 0..=(r+1) {
            for x in (5-r)..(15+r) {
                for y in (5-r)..(15+r) {
                    let mut ncount = 0;
                    for dz in -1i32..=1 {
                        for dx in -1i32..=1 {
                            for dy in -1i32..=1 {
                                if dx == 0 && dy == 0 && dz == 0 {continue;}
                                let nx = x+dx;
                                let ny = y+dy;
                                let nz = (z+dz).abs();
                                if inb(nx, 20) && inb(ny, 20) && inb(nz, 7) && space[nz as usize][nx as usize][ny as usize] {
                                    ncount += 1;
                                    if ncount > 3 { break; }
                                }
                            }
                            if ncount > 3 { break; }
                        }
                        if ncount > 3 { break; }
                    }
                    if space[z as usize][x as usize][y as usize] {
                        if ncount != 2 && ncount != 3 {
                            new_space[z as usize][x as usize][y as usize] = false;
                        }
                    }
                    else if ncount == 3 {
                        new_space[z as usize][x as usize][y as usize] = true;
                    }
                }
            }
        }
        space = new_space;
        // for zz in 0..=(r+1) {
        //     println!("Z = {}",zz);
        //     for i in 0..20 {
        //         println!("{:?}", space[zz as usize][i].iter().map(|b| *b as i32).collect::<Vec<_>>());
        //     }
        // }
    }
    let mut total = 0;
    for z in 0..=6 {
        for x in 0..20 {
            for y in 0..20 {
                if space[z][x][y] {
                    total += if z==0 {1} else {2};
                }
            }
        }
    }
    println!("Final solution: {}", total);
}

fn main() {
    sol1();
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let mut space = [[[[false; 20]; 20]; 7]; 7];
    for (li, l) in contents.lines().enumerate() {
        for (ci, c) in l.chars().enumerate() {
            if c =='#' {
                space[0][0][6+li][6+ci] = true;
            }
        }
    }
    for r in 0..6 {
        let mut new_space = space.clone();
        for w in 0..=(r+1) {
            for z in 0..=(r+1) {
                for x in (5-r)..(15+r) {
                    for y in (5-r)..(15+r) {
                        let mut ncount = 0;
                        for dw in -1i32..=1 {
                            for dz in -1i32..=1 {
                                for dx in -1i32..=1 {
                                    for dy in -1i32..=1 {
                                        if dw == 0 && dx == 0 && dy == 0 && dz == 0 {continue;}
                                        let nw = (w+dw).abs();
                                        let nx = x+dx;
                                        let ny = y+dy;
                                        let nz = (z+dz).abs();
                                        if inb(nx, 20) && inb(ny, 20) && inb(nz, 7) && inb(nw, 7) && space[nw as usize][nz as usize][nx as usize][ny as usize] {
                                            ncount += 1;
                                            if ncount > 3 { break; }
                                        }
                                    }
                                    if ncount > 3 { break; }
                                }
                                if ncount > 3 { break; }
                            }
                            if ncount > 3 { break; }
                        }
                        if space[w as usize][z as usize][x as usize][y as usize] {
                            if ncount != 2 && ncount != 3 {
                                new_space[w as usize][z as usize][x as usize][y as usize] = false;
                            }
                        }
                        else if ncount == 3 {
                            new_space[w as usize][z as usize][x as usize][y as usize] = true;
                        }
                    }
                }
            }
        }
        space = new_space;
        // for zz in 0..=(r+1) {
            
        //     for ww in 0..=(r+1) {
        //         println!("Z = {}, W = {}",zz, ww);
        //         for i in 0..20 {
        //             println!("{:?}", space[ww as usize][zz as usize][i].iter().map(|b| *b as i32).collect::<Vec<_>>());
        //         }
        //     }
        // }
    }
    let mut total = 0;
    for w in 0..=6 {
        for z in 0..=6 {
            for x in 0..20 {
                for y in 0..20 {
                    if space[w][z][x][y] {
                        total += (if z==0 {1} else {2}) * (if w==0 {1} else {2});
                    }
                }
            }
        }
    }
    println!("Final solution: {}", total);
}
