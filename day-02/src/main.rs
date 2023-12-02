use std::{env, fs};
use std::str::FromStr;

const RED_MAX: i32 = 12;
const GREEN_MAX: i32 = 13;
const BLUE_MAX: i32 = 14;

fn main() -> Result<(), ()> {
    let mut pwd = env::current_dir()
        .map_err(|err| eprintln!("{err}"))?;
    pwd.push("input.txt");

    let contents = fs::read_to_string(pwd.as_path())
        .map_err(|err| eprintln!("{err}"))?;

    let mut possible_games = Vec::new();

    for line in contents.lines() {
        let prefix_len = "Game ".chars().count();
        let game_number = &line[prefix_len..line.find(":").expect("well formed input")];

        let game = &line[line.find(":").expect("well formed input") + 2..];
        let sets = game.split(";");
        let mut is_possible = true;

        'set: for set in sets {
            let results = set.split(",");
            for result in results {
                let mut result = result.trim().split(" ");
                let count = i32::from_str(result.next().expect("well formed input").trim()).expect("well formed input");
                let color = result.next().expect("well formed input").trim();
                match color {
                    "red" => { is_possible = count <= RED_MAX },
                    "green" => { is_possible = count <= GREEN_MAX },
                    "blue" => { is_possible = count <= BLUE_MAX },
                    _ => panic!("unknown color encountered")
                }
                if !is_possible {
                    break 'set;
                }
            }
        }

        if is_possible {
            possible_games.push(game_number);
            println!("{game_number} is possible");
        }
    }
    let sum: i32 = possible_games.iter().map(|number| i32::from_str(number).unwrap()).sum();
    println!("sum is {sum}");
    return Ok(())
}
