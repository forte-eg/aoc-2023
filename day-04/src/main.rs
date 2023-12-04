use read_input::read_input;

fn main() -> Result<(), ()> {
    let contents = read_input()?;
    let sum = run(&contents)?;
    println!("sum: {sum}");
    Ok(())
}

fn run(input: &str) -> Result<i32, ()> {
    let lines = input.lines();

    let mut sum = 0;
    for line in lines {
        let line = &line[line.chars().position(|c| c == ':').unwrap()+2..];
        let parts: Vec<&str> = line.split("|").collect();
        let [winning, having] = parts[0..2] else { panic!() };

        let winning = to_ints(winning);
        let having = to_ints(having);

        let hits = having.iter().fold(0, |acc, e| if winning.contains(e) { acc + 1 } else { acc } );

        match hits {
            0 => {}
            _ => sum += 1 << (hits - 1)
        }
    }

    return Ok(sum)
}

fn to_ints(input: &str) -> Vec<i32> {
    input.split(" ")
        .filter(|s| s.trim() != "")
        .map(|s| i32::from_str_radix(s.trim(), 10).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input =
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
         Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
         Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
         Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
         Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
         Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(Ok(13), run(input));
    }
}