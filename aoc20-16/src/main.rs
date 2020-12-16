use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let mut nearby_found = false;
    let mut my_found = false;
    let mut my_ticket = vec!();
    let mut result1 = 0;
    let mut nearby = vec!();
    let mut limits = vec!();
    let mut total_min = 999;
    let mut total_max = 0;
    for l in contents.lines() {
        if l == "" {
            continue;
        }
        if !my_found {
            if l == "your ticket:" {
                my_found = true;
            }
            else {
                //println!("{}",l);
                let words: Vec<_> = l.split(": ").collect();
                let desc = words[0];
                let words: Vec<_> = words[1].split(" or ").collect();
                let w1: Vec<_> = words[0].split('-').collect();
                let w2: Vec<_> = words[1].split('-').collect();
                let min1 = w1[0].parse::<u32>().unwrap();
                total_min = std::cmp::min(total_min, min1);
                let max1 = w1[1].parse::<u32>().unwrap();
                total_max = std::cmp::max(total_max, max1);
                let min2 = w2[0].parse::<u32>().unwrap();
                total_min = std::cmp::min(total_min, min2);
                let max2 = w2[1].parse::<u32>().unwrap();
                total_max = std::cmp::max(total_max, max2);
                limits.push((desc, min1, max1, min2, max2));
            }
            continue;
        }

        if !nearby_found {
            if l == "nearby tickets:" {
                nearby_found = true;
            }
            else {
                let nbs: Vec<_> = l.split(',').collect();
                my_ticket = nbs.iter().map(|s| s.parse::<u32>().unwrap()).collect();
            }
            continue;
        }
        let nbs: Vec<_> = l.split(',').collect();
        let nbs: Vec<_> = nbs.iter().map(|s| s.parse::<u32>().unwrap()).collect();
        let mut valid = true;
        for nb in &nbs {
            if *nb < total_min || *nb > total_max {
                result1 += nb;
                valid = false;
            }
        }
        if valid {
            nearby.push(nbs);
        }
    }
    println!("Final solution1: {}", result1);
    let max_l = limits.len();
    let mut all_poss = vec!(); // all_poss[i][j] = is it possible for the i'th attribute to be writen at the j'th position
    // compute all_poss
    for i in 0..max_l {
        let mut poss = vec![true; max_l];
        let min1 = limits[i].1;
        let max1 = limits[i].2;
        let min2 = limits[i].3;
        let max2 = limits[i].4;
        for tic in &nearby {
            for j in 0..max_l {
                let val = tic[j];
                if !((min1 <= val && val <= max1) || (min2 <= val && val <= max2)) {
                    poss[j] = false;
                }
            }
        }
        all_poss.push(poss);
    }

    let mut todo = vec![true; max_l]; // is the position of the i'th value in the passports still to be found
    let mut done_poss = vec![false; max_l]; // is the i'th attribute (and therefore i'th entry in all_poss) already solved?
    let mut result2 = 1u64;
    while todo.contains(&true) {
        for i in 0..max_l {
            if done_poss[i] {continue;}
            let mut sol = None;
            let mut unique = true;
            // is there only a single possibility where the i'th attribute could be in the passports?
            for j in 0..max_l {
                if !todo[j] {continue;}
                if all_poss[i][j] {
                    if sol == None {sol = Some(j);}
                    else {
                        unique = false;
                        break;
                    }
                }
            }
            if unique {
                if let Some(v) = sol {
                    todo[v] = false;
                    done_poss[i] = true;
                    if limits[i].0.starts_with("departure") {
                        result2 *= my_ticket[v] as u64;
                    }
                    //println!("{}: {}",limits[i].0, my_ticket[v]);
                }
            }
        }
    }
    println!("Second solution: {}", result2);
}
