use read_input::read_input;

fn main() -> Result<(), ()> {
    let contents = read_input()?;
    let result: i64 = part1(&contents).iter().product();
    println!("silver: {result}");
    let result = part2(&contents)[0];
    println!("gold: {result}");
    Ok(())
}

fn part1(input: &str) -> Vec<i64> {
    let numbers: Vec<Vec<i64>> = input.lines()
        .map(|l| l.split_whitespace().skip(1).map(|s| s.parse::<i64>().unwrap()).collect())
        .collect();
    run(&numbers[0], &numbers[1])
}

fn part2(input: &str) -> Vec<i64> {
    let numbers: Vec<i64> = input.lines()
        .map(|l| l.split_whitespace().skip(1).collect::<String>())
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    run(&[numbers[0]], &[numbers[1]])
}

fn run(times: &[i64], dists: &[i64]) -> Vec<i64> {
    let mut wins: Vec<i64> = Vec::new();
    for (time, dist) in times.iter().zip(dists.iter()) {
        let mut w = 0;
        for wait in 0..*time {
            let t = time - wait;
            let v = wait;
            let s = v * t;
            if s > *dist {
                w += 1;
            }
        }
        wins.push(w);
    }
    wins
}

#[cfg(test)]
mod tests {
    use read_input::read_test_input;
    use super::*;

    #[test]
    fn example_p1() {
        let contents = read_test_input().unwrap();
        let wins = part1(&contents);
        let mut x = wins.iter();
        assert_eq!(Some(4), x.next().copied());
        assert_eq!(Some(8), x.next().copied());
        assert_eq!(Some(9), x.next().copied());
        assert_eq!(None, x.next())
    }
}