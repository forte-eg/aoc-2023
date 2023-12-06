use read_input::read_input;

fn main() -> Result<(), ()> {
    let contents = read_input()?;
    let result: i32 = run(&contents).iter().product();
    println!("{result}");
    Ok(())
}

fn run(input: &str) -> Vec<i32> {
    let numbers: Vec<Vec<i32>> = input.lines()
        .map(|l| l.split_whitespace().skip(1).map(|s| s.parse::<i32>().unwrap()).collect())
        .collect();

    let mut wins: Vec<i32> = Vec::new();
    for (time, dist) in numbers[0].iter().zip(numbers[1].iter()) {
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
        let wins = run(&contents);
        let mut x = wins.iter();
        assert_eq!(Some(4), x.next().copied());
        assert_eq!(Some(8), x.next().copied());
        assert_eq!(Some(9), x.next().copied());
        assert_eq!(None, x.next())
    }
}