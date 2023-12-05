// use std::collections::HashMap;
use read_input::read_input;

struct QuickMapEntry {
    diff: i64,
    lower: i64,
    upper: i64,
}

struct QuickMap {
    entries: Vec<QuickMapEntry>,
}

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
        let mut entries: Vec<_> = block.lines().skip(1)
            .map(|line| QuickMapEntry::parse(line))
            .collect();
        entries.sort_by(|a, b| a.lower.cmp(&b.lower));
        QuickMap { entries }
    }

    // binary search the entries
    fn get(&self, key: i64) -> i64 {
        let entries = &self.entries;
        let mut l = 0usize;
        let mut r = entries.len() - 1;

        while l <= r {
            let m = l + (r - l) / 2;
            let entry = &entries[m];
            if key >= entry.lower && key < entry.upper {
                return entry.diff + key;
            }
            if key >= entry.upper {
                l = m + 1;
            } else {
                if m == 0 {break;}
                r = m - 1;
            }
        }
        key
    }
}

// struct CachedSearch {
//     maps: Vec<QuickMap>,
//     cache: HashMap<(i64, QuickMap), i64>,
// }
//
// impl CachedSearch {
//     fn new(maps: Vec<QuickMap>) -> CachedSearch {
//         CachedSearch { maps, cache: HashMap::new() }
//     }
//
//     fn lookup(&self, key: i64) -> i64 {
//         let mut val = key;
//         for map in &self.maps {
//             val = map.get(val);
//             if self.cache[(val, map)]
//         }
//         val
//     }
// }

fn main() -> Result<(), ()> {
    let contents = read_input()?;
    let result = run(&contents)?;
    println!("gold: {result}");
    Ok(())
}

fn run(input: &str) -> Result<i64, ()> {
    let lines: Vec<_> = input.split("\n\n").collect();
    let seeds: Vec<_> = lines[0].split_whitespace().skip(1).map(|s| s.parse::<i64>().unwrap()).collect();
    let maps: Vec<_> = lines[1..].iter().map(|s| QuickMap::parse(s)).collect();

    let mut min = i64::MAX;
    for seed_chunk in seeds.chunks(2) {
        let [seed_0, count] = seed_chunk else { panic!() };
        for i in 0..*count {
            let mut location = seed_0 + i;
            for map in &maps {
                location = map.get(location);
            }
            if location < min {
                min = location;
            }
        }
    }

    Ok(min)
}

#[cfg(test)]
mod tests {
    use read_input::read_test_input;
    use super::*;

    #[test]
    fn example_p2() {
        let contents = read_test_input().unwrap();
        let result = run(&contents).unwrap();
        assert_eq!(46, result);
    }
}