use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let mut first = true;
    let mut val = 0;
    let mut busses = vec!();
    for l in contents.lines(){
        if first {
            val = l.parse::<u64>().unwrap();
            first = false;
        }
        else {
            let busstr = l.split(',');
            for bus in busstr {
                busses.push(if bus == "x" {None} else {Some(bus.parse::<u64>().unwrap())});
            }
        }
    }
    let mut min_wait = 9999999;
    let mut result = 0;
    let mut time = 0;
    let mut time_mult = 1;
    for (idx, bus) in busses.iter().enumerate() {
        if let Some(b) = bus {
            let wait = b - val%b;
            if wait < min_wait {
                min_wait = wait;
                result = b*wait;
                println!("Bus #{} waits {} minutes -> {}", b, wait, result);
            }
            while time%b != (b-((idx as u64)%b))%b {
                time += time_mult;
            }
            time_mult *= b;
            println!("BSF after bus #{} wants to wait {} min: t= {} - current_mult = {}", b, idx, time, time_mult);
        }
    }
}
