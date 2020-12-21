use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let mut allergens: HashMap<String, HashSet<String>> = HashMap::new();
    let mut all_ingred = HashMap::new();
    for l in contents.lines() {
        let words: Vec<_>= l.split(" (contains ").collect();
        let ingredients: HashSet<String>= words[0].split(' ').map(|s| s.to_string()).collect();
        let list_allergen = words[1].replace(')', "");
        for aller in list_allergen.split(", ") {
            if let Some(e) = allergens.get_mut(&aller.to_string()) {
                e.retain(|i| ingredients.contains(i));
            }
            else {
                allergens.insert(aller.to_string(), ingredients.clone());
            }
        }
        for i in ingredients {
            if let Some(v) = all_ingred.get_mut(&i) {
                *v += 1;
            }
            else {
                all_ingred.insert(i,1);
            }
        }
    }
    for (k, v) in &allergens {
        println!("{}: {:?}", k, v);
    }
    let mut result1 = 0;
    for (i, a) in &all_ingred {
        let mut found = false;
        for (_, v) in &allergens {
            if v.contains(i) {
                found = true;
                break;
            }
        }
        if !found {
            result1 += a;
        }
    }
    println!("{} ingredients cant contain allergens.", result1);

    let mut solution2 = vec!();
    while !allergens.is_empty()
    {
        let mut aller = String::new();
        let mut ingred = String::new();
        for (a, i) in &allergens {
            if i.len() == 1 {
                aller = a.clone();
                ingred = i.iter().next().unwrap().clone();
                break;
            }
        }
        allergens.remove(&aller);
        for (_, i) in & mut allergens {
            i.remove(&ingred);
        }
        solution2.push((aller, ingred));
    }
    solution2.sort();
    let mut result2 = solution2[0].1.clone();
    for i in 1..solution2.len() {
        result2.push(',');
        result2.push_str(&solution2[i].1);
    }
    println!("Second result: {}", result2);
}
