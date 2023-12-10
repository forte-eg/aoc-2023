use std::cmp::Ordering;
use std::collections::HashMap;
use read_input::read_input;

#[derive(Debug, Ord, Eq)]
struct Hand<'a> {
    cards: &'a str,
    bid: u64,
    strength: u32,
    label_strengths: [u32; 5],
}

impl<'a> Hand<'a> {
    fn parse(line: &'a str) -> Hand {
        let [cards, bid] = line.split_whitespace().collect::<Vec<_>>()[..] else { panic!() };
        Hand { cards , bid: bid.parse().unwrap(), strength: Hand::hand_strength(cards), label_strengths: Hand::label_strength(cards) }
    }

    fn hand_strength(cards: &str) -> u32 {
        let mut map: HashMap<char, u32> = HashMap::new();
        for card in cards.chars() {
            let old = map.get(&card).unwrap_or(&0);
            map.insert(card, *old + 1);
        }
        let mut groups = map.values().map(|i| *i).collect::<Vec<_>>();
        groups.sort_unstable_by(|a,b| b.cmp(a));
        return match groups[..] {
            [5] => 7,
            [4,..] => 6,
            [3,2,..] => 5,
            [3,..] => 4,
            [2, 2,..] => 3,
            [2,..] => 2,
            _ => 1
        }
    }

    fn label_strength(cards: &str) -> [u32; 5] {
        let mut strengths = [1u32; 5];
        let chars = cards.chars().collect::<Vec<_>>();
        for i in 0..5 {
            strengths[i] = match chars[..][i] {
                x if x.is_digit(10) => x.to_digit(10).unwrap(),
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => panic!()
            };
        }
        return strengths;
    }
}

impl<'a> PartialEq<Self> for Hand<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.cards.eq(other.cards)
    }
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.strength == other.strength {
            let mut i = 0;
            while self.label_strengths[i] == other.label_strengths[i] { i += 1 }
            return Some(self.label_strengths[i].cmp(&other.label_strengths[i]))
        }
        Some(self.strength.cmp(&other.strength))
    }
}

fn main() -> Result<(), ()> {
    let contents = read_input()?;
    let total_winings = run(&contents);
    println!("silver: {total_winings}");
    Ok(())
}

fn run(input: &str) -> u64 {
    let mut hands: Vec<_> = input.lines().map(|line| Hand::parse(line)).collect();
    hands.sort_unstable();
    hands.iter().enumerate().fold(0, |acc, (i, hand)| acc + ((i as u64 + 1) * hand.bid))
}

#[cfg(test)]
mod tests {
    use super::*;
    use read_input::read_test_input;

    #[test]
    fn example_p1() {
        let contents = read_test_input().unwrap();
        let total = run(&contents);
        assert_eq!(6440, total);
    }
}