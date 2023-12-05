use read_input::read_input;

struct QuickMapEntry {
    diff: i64,
    lower: i64,
    upper: i64,
}

struct QuickMap {
    entries: Vec<QuickMapEntry>
}

// 98 -> 50
// 99 -> 51

//  in(x) = x € [src..src+count]
// get(x) = (dst - src) + x  | in(x) == true
// get(x) = x                | in(x) == false

impl QuickMapEntry {
    fn parse(numbers: &str) -> QuickMapEntry {
        let numbers: Vec<_> = numbers.split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        let [dst, src, count] = numbers[..] else { panic!() };
        QuickMapEntry { diff: dst - src, lower: src, upper: src + count }
    }
}

impl QuickMap {
    fn parse(block: &str) -> QuickMap {
        let entries = block.lines().skip(1)
            .map(|line| QuickMapEntry::parse(line))
            .collect();
        QuickMap { entries }
    }

    // TODO: sorted + binary search?
    fn get(&self, key: i64) -> i64 {
        self.entries.iter()
            .find(|entry| key >= entry.lower && key < entry.upper)
            .map(|entry| entry.diff + key)
            .unwrap_or(key)
    }
}

fn main() -> Result<(), ()> {
    let contents = read_input()?;
    let result = run(&contents)?;
    println!("silver: {result}");
    Ok(())
}

fn run(input: &str) -> Result<i64, ()> {
    let lines: Vec<_> = input.split("\n\n").collect();

    let seeds: Vec<_> = lines[0].split_whitespace().skip(1).map(|s| s.parse::<i64>().unwrap()).collect();
    let maps: Vec<_> = lines[1..].iter()
        .map(|s| QuickMap::parse(s))
        .collect();

    let mut locations = Vec::new();
    for seed in seeds {
        let mut location = seed;
        for map in &maps {
            location = map.get(location);
        }
        locations.push(location);
    }

    Ok(*locations.iter().min().unwrap())
}

#[cfg(test)]
mod tests {
    use read_input::read_test_input;
    use super::*;

    #[test]
    fn example_p1() {
        let contents = read_test_input().unwrap();
        let result = run(&contents).unwrap();
        assert_eq!(35, result);
    }
}