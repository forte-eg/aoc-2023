use std::cmp;
use read_input::read_input;

struct Card {
    pub wins: usize
}

impl Card {
    fn parse(input: &str) -> Card {
        fn to_ints(input: &str) -> Vec<i32> {
            input.split_whitespace()
                .map(|s| i32::from_str_radix(s, 10).unwrap())
                .collect()
        }
        let line = &input[input.chars().position(|c| c == ':').unwrap()+2..];
        let parts: Vec<Vec<i32>> = line.split("|").into_iter().map(|s| to_ints(s)).collect();
        let [winning, having] = &parts[0..2] else { panic!() };
        let wins = having.iter()
            .filter(|i| winning.contains(i))
            .count();
        Card { wins }
    }
}

fn main() -> Result<(), ()> {
    let contents = read_input()?;
    let sum = run(&contents)?;
    println!("sum: {sum}");
    Ok(())
}

fn run(input: &str) -> Result<i32, ()> {
    let cards: Vec<Card> = input.lines()
        .map(|s| Card::parse(s))
        .collect();

    let sum = cards.len() + r_wins(&cards, 0, cards.len());
    Ok(sum as i32)
}

fn r_wins(cards: &Vec<Card>, mut i: usize, upper_bound: usize) -> usize {
    let mut wins = 0;
    while i < cmp::min(cards.len(), upper_bound) {
        let card_wins = cards[i].wins;
        wins += r_wins(cards, i + 1, i + 1 + card_wins);
        wins += card_wins;
        i += 1;
    }
    return wins;
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

        assert_eq!(Ok(30), run(input));
    }
}