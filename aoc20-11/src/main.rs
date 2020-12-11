use std::fs;

fn inb(val: i32, limit: i32) -> bool{
    val >= 0 && val < limit
}

fn get_nb_of_final_seats(seats : &Vec<Vec<Option<bool>>>, skip_vision : bool, vis_limit : u32) -> u32
{
    let mut seats = seats.clone();
    let rows = seats.len() as i32;
    let columns = seats[0].len() as i32;
    loop {
        let mut changes = vec!();
        for r in 0..rows {
            for c in 0..columns{
                let mut occupied = 0;
                if seats[r as usize][c as usize] == None {
                    continue;
                }
                for dr in -1..=1 {
                    if !inb(r+dr, rows) {continue;}
                    for dc in -1..=1 {
                        let mut nr = r+dr;
                        let mut nc = c+dc;
                        if !inb(nc, columns) {continue;}
                        if dc != 0 || dr != 0 {
                            while skip_vision && seats[nr as usize][nc as usize] == None && inb(nr+dr, rows) && inb(nc+dc, columns){
                                nr += dr;
                                nc += dc;
                            }
                            if Some(true) == seats[nr as usize][nc as usize] {
                                occupied += 1;
                            }
                        }
                    }
                }
                let cur_seat = & mut seats[r as usize][c as usize];
                if *cur_seat == Some(true) && occupied >= vis_limit {
                    changes.push((r,c));
                }
                else if *cur_seat == Some(false) && occupied ==0 {
                    changes.push((r,c));
                }
            }
        }
        if changes.len() == 0 {break;}
        for (r, c) in changes {
            if let Some(b) = seats[r as usize][c as usize] {
                seats[r as usize][c as usize] = Some(!b);
            }
        }
        // for row in &seats {
        //     let mut s = String::new();
        //     for val in row{
        //         s.push(match *val {None => '.', Some(true)=>'#', Some(false)=>'L'});
        //     }
        //     println!("{}", s);
        // }
        // println!();
    }
    seats.iter().fold(0, |acc, v| acc + v.iter().fold(0, |acc, val| if *val == Some(true) {acc + 1} else {acc}))
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let seats :Vec<Vec<_>> = contents.lines().map(|s| s.chars().map(|c| match c {'.' => None, 'L' => Some(false), _ => Some(true)}).collect()).collect();
    println!("First Solution: {}", get_nb_of_final_seats(&seats, false, 4));
    println!("Second Solution: {}", get_nb_of_final_seats(&seats, true, 5));
}
