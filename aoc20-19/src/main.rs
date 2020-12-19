use std::fs;
use std::collections::HashMap;

struct Rule {
    rule: Vec<Vec<u64>>,
    poss: Vec<String>
}

fn get_poss(rule_n: u64, rules: & mut HashMap<u64, Rule>) -> &Vec<String> {
    if rules[&rule_n].poss.len() == 0 {
        let mut poss = vec!();
        let rule = rules[&rule_n].rule.clone();
        for r in &rule {
            let mut poss_r = vec!();
            for rn in r {
                if poss_r.len() == 0 {
                    poss_r = get_poss(*rn, rules).clone();
                }
                else {
                    let comboposs = get_poss(*rn, rules);
                    let mut new_poss = vec!();
                    for cp in comboposs {
                        for p in &poss_r {        
                            let mut new_str = p.clone();
                            new_str.push_str(cp);
                            new_poss.push(new_str);
                        }
                    }
                    poss_r = new_poss;
                }
            }
            poss.append(& mut poss_r);
        }
        rules.insert(rule_n, Rule{rule, poss});
        println!("Processed rule #{}, found {} solutions.", rule_n, &rules[&rule_n].poss.len());
    }
    &rules[&rule_n].poss
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let mut rules = HashMap::new();
    let mut strings = vec!();
    for l in contents.lines() {
        if l.contains(':') {
            let words: Vec<_> = l.split(": ").collect();
            let rule_n = words[0].parse::<u64>().unwrap();
            let words: Vec<_> = words[1].split(" | ").collect();
            let mut rule = vec!();
            let mut poss = vec!();
            for w in words {
                if w.contains('"') {
                    poss.push(w.replace('"', ""));
                }
                else {
                    let subr : Vec<_> = w.split(' ').collect();
                    rule.push(subr.iter().map(|v| v.parse::<u64>().unwrap()).collect::<Vec<_>>());
                }
            }
            rules.insert(rule_n, Rule{rule, poss});
        }
        else {
            strings.push(String::from(l));
        }
    }
    let poss0 = get_poss(0, & mut rules).clone();
    let poss42 = get_poss(42, & mut rules).clone();
    let len42 = poss42[0].len();
    for p in &poss42 {
        if p.len() != len42 {panic!()};
    }
    println!("42 are {} characters long.", len42);
    let poss31 = get_poss(31, & mut rules).clone();
    let len31 = poss31[0].len();
    for p in &poss31 {
        if p.len() != len31 {panic!()};
    }
    println!("31 are {} characters long.", len31);
    let mut count1 = 0;
    let mut count2 = 0;
    for s in &strings {
        if poss0.contains(s) {
            //println!("{} found easily!", s);
            count1 += 1;
            count2 += 1;
        }
        else {
            let str_len = s.len();
            if str_len > len31+len42 && poss42.contains(&String::from(&s[0..len42])) && poss31.contains(&String::from(&s[str_len-len31..])){
                //println!("{} could be another match!", s);
                let mut is_match = false;
                let mut n42 = 1;
                loop {
                    n42 += 1;
                    if str_len < len31+len42*n42 {
                        //println!("{} failed because too short.", s);
                        break;
                    }
                    if !poss42.contains(&String::from(&s[len42*(n42-1)..len42*n42])) {
                        //println!("{} failed because begin {} doesn't match", s, n42);
                        break;
                    }
                    if (str_len - len42*n42)%len31 != 0 || (str_len - len42*n42)/len31 >= n42{
                        continue;
                    }
                    else {
                        is_match = true;
                        for n31 in 0..(str_len - len42*n42)/len31 - 1 {
                            if !poss31.contains(&String::from(&s[len42*n42+len31*n31..len42*n42+len31*(n31+1)])) {
                                is_match = false;
                                break;
                            }
                        }
                        
                        if is_match { break;}
                    }
                    
                }
                if is_match {
                    //println!("{} is another match!", s);
                    count2 += 1;
                }
            }
        }
    }
    println!("Solution 1: {}", count1);
    println!("Solution 2: {}", count2);
}
