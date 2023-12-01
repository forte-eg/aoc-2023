use std::env;
use std::fs;

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
        for char in line.chars() {
            char.to_digit(10).map(|digit| digits.push(digit));
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
