use read_input::read_input;
use std::cmp;

fn main() -> Result<(), ()> {
    let contents = read_input()?;
    return run(&contents).map(|sum| println!("sum: {sum}"));
}

fn run(input: &str) -> Result<i32, ()> {
    let lines: Vec<&str> = input.lines().collect();

    let mut part_numbers = Vec::new();

    for row in 0..lines.len() {
        let line = lines[row];
        let mut offset: usize = 0;

        if row > 0 {println!("   {}", lines[row-1])}
        println!(">> {line} <<");
        if row + 1 < lines.len() {println!("   {}", lines[row+1])}

        while offset < line.len() {
            let (number, new_offset) = find_number(line, offset);
            if number == "" { break }
            if is_part_number(&lines, row, new_offset, number.len()) {
                println!("found {number}");
                part_numbers.push(number);
            }
            offset = new_offset + number.len();
        }
        println!();
    }
    let a: Vec<i32> = part_numbers.iter().map(|n| i32::from_str_radix(n, 10).unwrap()).collect();
    println!("{a:?}\n\nlength={}\n", a.len());

    let sum: i32 = a.iter().sum();
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

fn is_part_number(lines: &Vec<&str>, row: usize, offset: usize, length: usize) -> bool {
    let current = scan(lines[row], offset, length);
    let top = row > 0 && scan(lines[row-1], offset, length);
    let bottom = row + 1 < lines.len() && scan(lines[row+1], offset, length);

    return current || top || bottom;
}

fn scan(line: &str, offset: usize, length: usize) -> bool {
    let from = if offset == 0 { 0 } else { offset - 1 };
    let to = cmp::min(line.len(), offset + length + 1);
    let slice = &line[from..to];
    return slice.chars().any(|c| is_symbol(c));
}

fn is_symbol(c: char) -> bool {
    c != '.' && c.is_ascii_punctuation()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = vec![
            ".....",
            ".123.",
            ".....",
        ];
        assert!(!is_part_number(&input, 1, 1, 3))
    }

    #[test]
    fn test1() {
        let input = vec![
            "*....",
            ".123.",
            ".....",
        ];
        assert!(is_part_number(&input, 1, 1, 3))
    }

    #[test]
    fn test2() {
        let input = vec![
            ".*...",
            ".123.",
            ".....",
        ];
        assert!(is_part_number(&input, 1, 1, 3))
    }

    #[test]
    fn test3() {
        let input = vec![
            "..*..",
            ".123.",
            ".....",
        ];
        assert!(is_part_number(&input, 1, 1, 3))
    }

    #[test]
    fn test4() {
        let input = vec![
            "...*.",
            ".123.",
            ".....",
        ];
        assert!(is_part_number(&input, 1, 1, 3))
    }

    #[test]
    fn test5() {
        let input = vec![
            "....*",
            ".123.",
            ".....",
        ];
        assert!(is_part_number(&input, 1, 1, 3))
    }

    #[test]
    fn test6() {
        let input = vec![
            ".....",
            "*123.",
            ".....",
        ];
        assert!(is_part_number(&input, 1, 1, 3))
    }

    #[test]
    fn test7() {
        let input = vec![
            ".....",
            ".123*",
            ".....",
        ];
        assert!(is_part_number(&input, 1, 1, 3))
    }

    #[test]
    fn test8() {
        let input = vec![
            ".....",
            ".123.",
            "*....",
        ];
        assert!(is_part_number(&input, 1, 1, 3))
    }

    #[test]
    fn test9() {
        let input = vec![
            ".....",
            ".123.",
            ".*...",
        ];
        assert!(is_part_number(&input, 1, 1, 3))
    }

    #[test]
    fn test10() {
        let input = vec![
            ".....",
            ".123.",
            "..*..",
        ];
        assert!(is_part_number(&input, 1, 1, 3))
    }

    #[test]
    fn test11() {
        let input = vec![
            ".....",
            ".123.",
            "...*.",
        ];
        assert!(is_part_number(&input, 1, 1, 3))
    }

    #[test]
    fn test12() {
        let input = vec![
            ".....",
            ".123.",
            "....*",
        ];
        assert!(is_part_number(&input, 1, 1, 3))
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
        assert!(!is_part_number(&input, 2, 2, 3))
    }

    #[test]
    fn test14() {
        let input = vec![
            "*******",
            "*.....*",
            "*.123.*",
            "*.....*",
            "*******",
        ];
        let (number, new_offset) = find_number(input[2], 0);
        assert_eq!(number, "123");
        assert_eq!(new_offset, 2);

        assert_eq!("", find_number(input[0], 0).0);
        assert_eq!("", find_number(input[1], 0).0);
        assert_eq!("", find_number(input[3], 0).0);
        assert_eq!("", find_number(input[4], 0).0);
    }

    #[test]
    fn test15() {
        let input = vec![
            "*****",
            "....*",
            "123.*",
            "....*",
            "*****",
        ];
        assert!(!is_part_number(&input, 2, 0, 3))
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
        assert!(!is_part_number(&input, 2, 2, 3))
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
        assert_eq!(Ok(4361), run(input))
    }

    #[test]
    fn b_case_1() {
        let input =
        "..123..\n\
         456*...\n\
         .......";
        assert_eq!(Ok(579), run(input))
    }

    #[test]
    fn b_case_2() {
        let input =
        ".....\n\
         5*4*5\n\
         .....";
        assert_eq!(Ok(14), run(input))
    }
}