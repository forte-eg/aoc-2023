use std::env;
use std::fs;

struct X {
    spelling: &'static str,
    value: u32,
}

const SPELLED: [X; 9] = [
    X { spelling: "one", value: 1 },
    X { spelling: "two", value: 2 },
    X { spelling: "three", value: 3 },
    X { spelling: "four", value: 4 },
    X { spelling: "five", value: 5 },
    X { spelling: "six", value: 6 },
    X { spelling: "seven", value: 7 },
    X { spelling: "eight", value: 8 },
    X { spelling: "nine", value: 9 },
];



fn digit(slice: &str) -> Option<u32> {
    if let Some(digit) = slice.chars().nth(0).map(|char| char.to_digit(10)) {
        if digit.is_some() {
            return digit;
        }
    }

    for X {spelling, value} in SPELLED {
        if spelling.len() <= slice.len() {
            let sub_slice = &slice[0..spelling.len()];
            if sub_slice.eq(spelling) {
                return Some(value);
            }
        }
    }

    return None;
}

fn main() -> Result<(), ()> {
    let mut pwd = env::current_dir()
        .map_err(|err| eprintln!("{err}"))?;
    pwd.push("input");

    let contents = fs::read_to_string(pwd.as_path())
        .map_err(|err| eprintln!("{err}"))?;

    let mut codes: Vec<u32> = Vec::new();
    let mut digits: Vec<u32> = Vec::new();
    for line in contents.split("\n") {
        digits.clear();
        for i in 0..line.len() {
            let slice = &line[i..line.len()];
            if let Some(digit) = digit(slice) {
                digits.push(digit);
            }
        }
        let mut code: u32 = 0;
        digits.first().map(|digit| code = 10 * *digit);
        digits.last().map(|digit| code += *digit);
        codes.push(code);
        println!("{:?} -> {code}", digits);
    }
    let sum: u32 = codes.iter().sum();
    println!("{sum}");
    Ok(())
}
