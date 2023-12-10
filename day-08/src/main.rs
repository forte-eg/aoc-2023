const ARR_SZ: usize = 26 * 26 * 26;

fn main() {
    let binding = read_input::read_input().unwrap();
    let content = binding.split("\n\n").collect::<Vec<_>>();
    let [instructions, map] = content[..] else { panic!() };
    let steps = run(instructions.as_bytes(), map);
    println!("silver: {steps}");
}

fn run(instructions: &[u8], map: &str) -> u64 {
    let mut lut: [(usize, usize); ARR_SZ] = [(0, 0); ARR_SZ];
    let starting_nodes = populate(map, &mut lut);

    let mut steps = 0;
    let mut location = hash(&[b'A', b'A', b'A']);
    let end = hash(&[b'Z', b'Z', b'Z']);

    while location != end {
        let (left, right) = lut[location];
        let next_turn = steps % instructions.len();
        location = match instructions[next_turn] {
            b'L' => left,
            b'R' => right,
            _ => panic!()
        };
        steps += 1;
    }
    return steps as u64;
}

fn populate(data: &str, lut: &mut [(usize, usize); ARR_SZ]) -> Vec<usize> {
    let mut starting_nodes = Vec::new();
    let lines = data.split("\n").map(|s| s.as_bytes()).collect::<Vec<_>>();
    for line in lines {
        let k = &line[0..=2];
        let l = &line[7..=9];
        let r = &line[0xC..=0xE];
        let key_hash = hash(k);
        lut[key_hash] = (hash(l), hash(r));
        if k[2] == b'A' {
            starting_nodes.push(key_hash);
        }
    }
    return starting_nodes;
}

#[inline(always)]
fn hash(key: &[u8]) -> usize {
    let [a, b, c] = key else { panic!() };
    let x = b'A' as usize;
    let d = ((26 * 26) * (*a as usize - x)) + (26 * (*b as usize - x)) + (*c as usize - x);
    println!("{key:?} -> {d}");
    d
}

#[cfg(test)]
mod tests {
    use super::*;
    use read_input::read_test_input;

    #[test]
    fn ex_1() {
        let binding = read_test_input().unwrap();
        let content = binding.split("\n\n").collect::<Vec<_>>();
        let [instructions, map] = content[..] else { panic!() };
        let steps = run(instructions.as_bytes(), map);
        // assert_eq!(2, steps);
        assert_eq!(6, steps);
    }
}