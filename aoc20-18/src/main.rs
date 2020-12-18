use std::fs;

#[derive (Debug, PartialEq)]
enum Bl {
    Val(u64),
    Mul,
    Add,
    Bo,
    Bc
}

fn idx_matching_brace(seq: &[Bl], idx_open: usize) -> usize
{
    let mut open = 0;
    let mut result = 0;
    for i in idx_open..seq.len() {
        match seq[i] {
            Bl::Bo => open += 1,
            Bl::Bc => open -= 1,
            _ => ()
        }
        if open == 0 {
            result = i;
            break;
        }
    }
    result
}

fn get_first_val(seq: &[Bl], add_binds: bool) -> (u64, usize)
{
    match seq[0] {
        Bl::Val(v) => (v, 1),
        Bl::Bo => {
            let idx = idx_matching_brace(&seq, 0);
            (eval(&seq[1..idx], add_binds), idx +1)
        }
        _ => panic!()
    }
}

fn eval(seq: &[Bl], add_binds: bool) -> u64 {
    //println!("{:?}", seq);
    let (mut val1, mut idx) = get_first_val(seq, add_binds);
    while idx < seq.len()
    {
        let op = &seq[idx];
        idx += 1;
        let (mut val2, new_idx) = get_first_val(&seq[idx..], add_binds);
        idx += new_idx;
        val1 = match op {
            Bl::Add => val1 + val2,
            Bl::Mul => {while add_binds && idx < seq.len() && Bl::Add == seq[idx] {
                idx += 1;
                let (val3, new_idx) = get_first_val(&seq[idx..], add_binds);
                idx += new_idx;
                val2 += val3;
            }
                val1 * val2},
            _ => panic!()
        }
    }
    
    val1
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let mut result1 = 0;
    let mut result2 = 0;
    for l in contents.lines() {
        let mut bls = vec!();
        for c in l.chars() {
            bls.push(match c {
                ' ' => continue,
                '*' => Bl::Mul,
                '+' => Bl::Add,
                '(' => Bl::Bo,
                ')' => Bl::Bc,
                num => Bl::Val(num.to_digit(10).unwrap() as u64)
            });
        }
        let loc_res1 = eval(&bls[..], false);
        let loc_res2 = eval(&bls[..], true);
        //  println!("{} = {}", l, loc_res2);
        result1 += loc_res1;
        result2 += loc_res2;
    }
    println!("First result: {}", result1);
    println!("Second result: {}", result2);
}
