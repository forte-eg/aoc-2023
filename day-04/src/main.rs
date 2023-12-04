use read_input::read_input;

struct Card {
    pub wins: usize
}

impl Card {
    fn parse(input: &str) -> Card {
        let line = &input[input.chars().position(|c| c == ':').unwrap()+2..];
        let parts: Vec<Vec<&str>> = line.split("|").map(|s| s.split_whitespace().collect()).collect();
        let [winning, having] = &parts[0..2] else { panic!() };
        let wins = having.iter().filter(|i| winning.contains(i)).count();
        Card { wins }
    }
}

fn main() -> Result<(), ()> {
    let contents = read_input()?;
    let cards = contents.lines().map(|s| Card::parse(s)).collect();

    let silver = part1(&cards)?;
    println!("silver: {silver}");

    let gold = part2(&cards)?;
    println!("gold: {gold}");

    Ok(())
}

fn part1(cards: &Vec<Card>) -> Result<i32, ()> {
    let result = cards.iter()
        .map(|c| if c.wins > 0 { 1 << c.wins - 1 } else { 0 })
        .sum::<i32>() ;
    Ok(result)
}

fn part2(cards: &Vec<Card>) -> Result<i32, ()> {
    let mut sum = 0;
    for i in 0..cards.len() {
        sum += process_card(&cards, i);
    }
    Ok(sum as i32)
}

fn process_card(cards: &Vec<Card>, i: usize) -> usize {
    match cards[i].wins {
        0 => { 1 },
        wins => {
            let mut cs = 1;
            for j in i+1..i+1+wins {
                if j >= cards.len() { break; }
                cs += process_card(cards, j)
            }
            cs
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str =
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
         Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
         Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
         Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
         Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
         Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn example1() {
        let cards = INPUT.lines().map(|s| Card::parse(s)).collect();
        assert_eq!(Ok(13), part1(&cards));
    }

    #[test]
    fn example2() {
        let cards = INPUT.lines().map(|s| Card::parse(s)).collect();
        assert_eq!(Ok(30), part2(&cards));
    }
}