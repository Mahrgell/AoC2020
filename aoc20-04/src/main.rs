use std::fs;
use regex::Regex;

fn str_in_bounds(s: &str, lower: i32, higher: i32) -> bool
{
    match s.parse::<i32>(){
        Ok(y) => y>=lower && y<=higher,
        _ => false,
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file.");
    let mut fields_valid: u8 = 1;
    let mut fields_valid_value: u8 = 1;
    let mut valid = 0;
    let mut valid_value = 0;
    for l in contents.lines() {
        if l == "" {
            if fields_valid == 0b11111111{
                valid += 1;
            }
            if fields_valid_value == 0b11111111{
                valid_value += 1;
            }
            
            fields_valid = 1;
            fields_valid_value = 1;
            continue;
        }
        for word in l.split_whitespace() {
            let (code, value) = word.split_at(4);
            match code {
                "byr:" => {
                    fields_valid |= 1<<1;
                    if str_in_bounds(&value, 1920, 2002) {
                        fields_valid_value |= 1<<1;
                    }
                },
                "iyr:" => {
                    fields_valid |= 1<<2;
                    if str_in_bounds(&value, 2010, 2020) {
                        fields_valid_value |= 1<<2;
                    }
                },
                "eyr:" => {
                    fields_valid |= 1<<3;
                    if str_in_bounds(&value, 2020, 2030) {
                        fields_valid_value |= 1<<3;
                    }
                },
                "hgt:" => {
                    fields_valid |= 1<<4;
                    let mut min_val = 1;
                    let mut max_val = 0;
                    let mut nb: String = "".to_string();
                    if value.contains("cm")
                    {
                        nb = value.replace("cm", "");
                        min_val = 150;
                        max_val = 193;
                    }
                    else if value.contains("in")
                    {
                        nb = value.replace("in", "");
                        min_val = 59;
                        max_val = 76;
                    }
                    if str_in_bounds(&nb, min_val, max_val) {
                        fields_valid_value |= 1<<4;
                    }
                },
                "hcl:" => {
                    fields_valid |= 1<<5;
                    let re = Regex::new(r"#((\d|[a-f]){6})").unwrap();
                    if re.is_match(value) {
                        fields_valid_value |= 1<<5;
                    }
                },
                "ecl:" => {
                    fields_valid |= 1<<6;
                    if (value == "amb") || 
                        (value == "blu") ||
                        (value == "brn") ||
                        (value == "gry") ||
                        (value == "grn") ||
                        (value == "hzl") ||
                        (value == "oth"){
                        fields_valid_value |= 1<<6;
                    }
                },
                "pid:" => {
                    fields_valid |= 1<<7;
                    let re = Regex::new(r"(\d{9})").unwrap();
                    if re.is_match(value) {
                        fields_valid_value |= 1<<7;
                    }
                },
                "cid:" => {
                    fields_valid |= 1<<0;
                    fields_valid_value |= 1<<0;
                },
                _ => panic!(),
            }
        }
    }
    println!("{} passports contain necessary values.", valid);
    println!("{} passports contain valid values.", valid_value);
}
