use std::{env, fs};
use std::str::FromStr;

struct ColorFrequency {
    red: i32,
    green: i32,
    blue: i32,
}

fn main() -> Result<(), ()> {
    let mut pwd = env::current_dir()
        .map_err(|err| eprintln!("{err}"))?;
    pwd.push("input.txt");

    let contents = fs::read_to_string(pwd.as_path())
        .map_err(|err| eprintln!("{err}"))?;

    let mut powers = Vec::new();

    for line in contents.lines() {
        let game = &line[line.find(":").expect("well formed input") + 2..];
        let sets = game.split(";");

        let mut freq = ColorFrequency { red: 0, green: 0, blue: 0 };

        for set in sets {
            let outcomes = set.split(",");
            for outcome in outcomes {
                let mut outcome = outcome.trim().split(" ");
                let count = i32::from_str(outcome.next().expect("well formed input").trim())
                    .expect("well formed input");
                let color = outcome.next().expect("well formed input").trim();
                match color {
                    "red" if count > freq.red => { freq.red = count },
                    "green" if count > freq.green => { freq.green = count },
                    "blue" if count > freq.blue => { freq.blue = count },
                    _ => {}
                }
            }
        }

        let power = freq.red * freq.green * freq.blue;
        powers.push(power);
    }
    let sum: i32 = powers.iter().sum();
    println!("sum of powers is {sum}");
    return Ok(())
}
