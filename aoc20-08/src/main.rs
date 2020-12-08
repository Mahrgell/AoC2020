use std::fs;

#[derive(Debug, Copy, Clone)]
enum Op{
    Acc(i32),
    Jmp(i32),
    Nop(i32)
}

fn line_to_op(l: &str) -> Op {
    let words : Vec<_>= l.split(' ').collect();
    let val = words[1].parse::<i32>().unwrap();
    match words[0] {
        "nop" => Op::Nop(val),
        "acc" => Op::Acc(val),
        "jmp" => Op::Jmp(val),
        _ => panic!()
    }
}

fn run_ops(ops: &Vec<Op>, replace_pos: Option<i32>) -> (bool, i32)
{
    let mut visited = vec![false; ops.len()];
    let mut val = 0;
    let mut pos = 0i32;
    let mut success = false;
    while !visited[pos as usize] {
        visited[pos as usize] = true;
        let op = if Some(pos) == replace_pos {
            match ops[pos as usize] {
                Op::Nop(v) => Op::Jmp(v),
                Op::Jmp(v) => Op::Nop(v),
                _ => break,
            }
        }
        else {
            ops[pos as usize]
        };
        match op {
            Op::Acc(v) => { val += v; pos += 1;},
            Op::Jmp(v) => pos += v,
            Op::Nop(_) => pos += 1,
        }
        if pos < 0 || pos > ops.len() as i32 {
            break;
        }
        if pos == ops.len() as i32 {
            success = true;
            break;
        }
    }
    (success, val)
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let ops : Vec<_>= contents.lines().map(line_to_op).collect();
    //println!("{:?}", ops);
    let (_, val) = run_ops(&ops, None);
    println!("Final stack value without replaced op: {}", val);
    for i in 0..ops.len() {
        if let (true, val) = run_ops(&ops, Some(i as i32)) {
            println!("Replace instruction #{} successfully to get result {}.", i, val);
        }
    }
}
