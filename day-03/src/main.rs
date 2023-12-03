use read_input::read_input;
use std::{cmp, usize};
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Gear {
    line: usize,
    offset: usize,
}

fn main() -> Result<(), ()> {
    let contents = read_input()?;
    return run(&contents).map(|sum| println!("sum: {sum}"));
}

fn run(input: &str) -> Result<i32, ()> {
    let lines: Vec<&str> = input.lines().collect();

    let mut part_map: HashMap<Gear, Vec<i32>> = HashMap::new();

    for row in 0..lines.len() {
        let line = lines[row];
        let mut offset: usize = 0;

        if row > 0 { println!("   {}", lines[row-1]) }
        println!(">> {line} <<");
        if row + 1 < lines.len() { println!("   {}", lines[row+1]) }

        while offset < line.len() {
            let (number, new_offset) = find_number(line, offset);
            if number == "" { break }
            if let Some(gears) = is_part_number(&lines, row as i32, new_offset, number.len()) {
                println!("found {number}");
                for gear in &gears {
                    let parts = part_map.entry(*gear).or_insert_with(|| Vec::new());
                    parts.push(i32::from_str_radix(number, 10).unwrap());
                }
            }
            offset = new_offset + number.len();
        }
        println!();
    }

    let gears: HashMap<Gear, Vec<i32>> = part_map.into_iter()
        .filter(|(_, numbers)| numbers.len() >= 2)
        .collect();

    println!("{gears:?}");

    let sum = gears.into_iter()
        .map(|(_, numbers)| numbers.iter().product::<i32>())
        .sum();

    return Ok(sum);
}

fn find_number(line: &str, offset: usize) -> (&str, usize) {
    let chars: Vec<char> = line.chars().collect();

    let mut new_offset = offset;
    let mut slice = "";

    for i in offset..chars.len() {
        if !chars[i].is_digit(10) { continue; }
        new_offset = i;
        break;
    }

    for i in new_offset..chars.len() {
        if chars[i].is_digit(10) {
            slice = &line[new_offset..=i];
            continue;
        }
        break;
    }

    return (slice, new_offset)
}

fn is_part_number(lines: &Vec<&str>, row: i32, offset: usize, length: usize) -> Option<Vec<Gear>> {
    let mut current = scan(lines, row, offset, length);
    let mut top = scan(lines, row - 1 , offset, length);
    let mut bottom = scan(lines, row + 1, offset, length);

    let mut all: Vec<Gear> = Vec::new();
    all.append(&mut current);
    all.append(&mut top);
    all.append(&mut bottom);

    return if all.is_empty() { None } else { Some(all) }
}

fn scan(lines: &Vec<&str>, row: i32, offset: usize, length: usize) -> Vec<Gear> {
    if row < 0 || row >= lines.len() as i32 {
        return Vec::new();
    }

    let line = lines[row as usize];
    let from = cmp::max(0i32, offset as i32 - 1) as usize;
    let to = cmp::min(line.len(), offset + length + 1);
    let slice = &line[from..to];

    return slice.chars().enumerate()
        .filter(|(_, c)| *c == '*')
        .map(|(i, _)| Gear { line: row as usize, offset: from + i })
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test() {
        let input = vec![
            ".....",
            ".123.",
            ".....",
        ];
        assert_eq!(None, is_part_number(&input, 1, 1, 3))
    }

    #[test]
    fn test1() {
        let input = vec![
            "*....",
            ".123.",
            ".....",
        ];
        let gears = is_part_number(&input, 1, 1, 3).unwrap();
        assert_eq!(1, gears.len());
        assert_eq!(Gear { line: 0, offset: 0 }, *gears.get(0).unwrap());
    }

    #[test]
    fn test2() {
        let input = vec![
            ".*...",
            ".123.",
            ".....",
        ];
        let gears = is_part_number(&input, 1, 1, 3).unwrap();
        assert_eq!(1, gears.len());
        assert_eq!(Gear { line: 0, offset: 1 }, *gears.get(0).unwrap());
    }

    #[test]
    fn test3() {
        let input = vec![
            "..*..",
            ".123.",
            ".....",
        ];
        let gears = is_part_number(&input, 1, 1, 3).unwrap();
        assert_eq!(1, gears.len());
        assert_eq!(Gear { line: 0, offset: 2 }, *gears.get(0).unwrap());
    }

    #[test]
    fn test4() {
        let input = vec![
            "...*.",
            ".123.",
            ".....",
        ];
        let gears = is_part_number(&input, 1, 1, 3).unwrap();
        assert_eq!(1, gears.len());
        assert_eq!(Gear { line: 0, offset: 3 }, *gears.get(0).unwrap());
    }

    #[test]
    fn test5() {
        let input = vec![
            "....*",
            ".123.",
            ".....",
        ];
        let gears = is_part_number(&input, 1, 1, 3).unwrap();
        assert_eq!(1, gears.len());
        assert_eq!(Gear { line: 0, offset: 4 }, *gears.get(0).unwrap());
    }

    #[test]
    fn test6() {
        let input = vec![
            ".....",
            "*123.",
            ".....",
        ];
        let gears = is_part_number(&input, 1, 1, 3).unwrap();
        assert_eq!(1, gears.len());
        assert_eq!(Gear { line: 1, offset: 0 }, *gears.get(0).unwrap());
    }

    #[test]
    fn test7() {
        let input = vec![
            ".....",
            ".123*",
            ".....",
        ];
        let gears = is_part_number(&input, 1, 1, 3).unwrap();
        assert_eq!(1, gears.len());
        assert_eq!(Gear { line: 1, offset: 4 }, *gears.get(0).unwrap());
    }

    #[test]
    fn test8() {
        let input = vec![
            ".....",
            ".123.",
            "*....",
        ];
        let gears = is_part_number(&input, 1, 1, 3).unwrap();
        assert_eq!(1, gears.len());
        assert_eq!(Gear { line: 2, offset: 0 }, *gears.get(0).unwrap());
    }

    #[test]
    fn test9() {
        let input = vec![
            ".....",
            ".123.",
            ".*...",
        ];
                let gears = is_part_number(&input, 1, 1, 3).unwrap();
        assert_eq!(1, gears.len());
        assert_eq!(Gear { line: 2, offset: 1 }, *gears.get(0).unwrap());
    }

    #[test]
    fn test10() {
        let input = vec![
            ".....",
            ".123.",
            "..*..",
        ];
                let gears = is_part_number(&input, 1, 1, 3).unwrap();
        assert_eq!(1, gears.len());
        assert_eq!(Gear { line: 2, offset: 2 }, *gears.get(0).unwrap());
    }

    #[test]
    fn test11() {
        let input = vec![
            ".....",
            ".123.",
            "...*.",
        ];
                let gears = is_part_number(&input, 1, 1, 3).unwrap();
        assert_eq!(1, gears.len());
        assert_eq!(Gear { line: 2, offset: 3 }, *gears.get(0).unwrap());
    }

    #[test]
    fn test12() {
        let input = vec![
            ".....",
            ".123.",
            "....*",
        ];
                let gears = is_part_number(&input, 1, 1, 3).unwrap();
        assert_eq!(1, gears.len());
        assert_eq!(Gear { line: 2, offset: 4 }, *gears.get(0).unwrap());
    }

    #[test]
    fn test13() {
        let input = vec![
            "*******",
            "*.....*",
            "*.123.*",
            "*.....*",
            "*******",
        ];
        assert_eq!(None, is_part_number(&input, 2, 2, 3))
    }

    // #[test]
    // fn test14() {
    //     let input = vec![
    //         "*******",
    //         "*.....*",
    //         "*.123.*",
    //         "*.....*",
    //         "*******",
    //     ];
    //     let (number, new_offset) = find_number(input[2], 0);
    //     assert_eq!(number, "123");
    //     assert_eq!(new_offset, 2);
    //
    //     assert_eq!("", find_number(input[0], 0).0);
    //     assert_eq!("", find_number(input[1], 0).0);
    //     assert_eq!("", find_number(input[3], 0).0);
    //     assert_eq!("", find_number(input[4], 0).0);
    // }
    //
    #[test]
    fn test15() {
        let input = vec![
            "*****",
            "....*",
            "123.*",
            "....*",
            "*****",
        ];
        assert_eq!(None, is_part_number(&input, 2, 0, 3))
    }

    #[test]
    fn test16() {
        let input = vec![
            "*****",
            "*....",
            "*.123",
            "*....",
            "*****",
        ];
        assert_eq!(None, is_part_number(&input, 2, 2, 3))
    }

    #[test]
    fn test_example() {
        let input =
            "467..114..\n\
             ...*......\n\
             ..35..633.\n\
             ......#...\n\
             617*......\n\
             .....+.58.\n\
             ..592.....\n\
             ......755.\n\
             ...$.*....\n\
             .664.598..";
        assert_eq!(Ok(467835), run(input))
    }

    #[test]
    fn b_case_1() {
        let input =
        "..123..\n\
         456*...\n\
         .......";
        assert_eq!(Ok(56088), run(input))
    }

    #[test]
    fn b_case_2() {
        let input =
        ".....\n\
         5*4*5\n\
         .....";
        assert_eq!(Ok(40), run(input))
    }
}