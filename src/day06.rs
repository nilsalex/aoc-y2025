extern crate test;

const INPUT: &[u8] = include_bytes!("../inputs/day06.txt");

const POWERS_OF_TEN: [usize; 8] = [1, 10, 100, 1000, 10000, 100000, 1000000, 10000000];

fn usize_from_bytes(bytes: &[u8]) -> usize {
    bytes.iter().rev().enumerate().fold(0, |acc, (ix, x)| {
        acc + (x & 0x0f) as usize * POWERS_OF_TEN[ix]
    })
}

pub fn part1(input: &[u8]) -> usize {
    let mut nums: Vec<Vec<usize>> = vec![];
    let mut ops: Vec<u8> = vec![];

    let mut lines = input.split(|c| *c == b'\n');

    loop {
        let line = lines.next().unwrap();
        if line.iter().any(|c| *c != b' ' && (*c < b'0' || *c > b'9')) {
            ops = line.iter().cloned().filter(|c| *c != b' ').collect();
            break;
        } else {
            nums.push(
                line.split(|c| *c == b' ')
                    .filter(|l| !l.is_empty())
                    .map(usize_from_bytes)
                    .collect(),
            );
        }
    }

    let mut result = 0;

    for (ix, o) in ops.iter().enumerate() {
        let r = match o {
            b'*' => nums.iter().fold(1, |acc, n| acc * n[ix]),
            b'+' => nums.iter().fold(0, |acc, n| acc + n[ix]),
            _ => panic!(),
        };
        result += r
    }

    result
}

pub fn part2(input: &[u8]) -> usize {
    let rows = input
        .split(|c| *c == b'\n')
        .filter(|l| !l.is_empty())
        .count();
    let cols = input.iter().take_while(|&c| *c != b'\n').count() + 1;

    let ops_offset = (rows - 1) * cols;

    let mut op_pointer = ops_offset;
    let mut op = input[ops_offset];
    let mut cur_res = match op {
        b'*' => 1,
        b'+' => 0,
        _ => panic!(),
    };

    let mut res = 0;

    loop {
        if op_pointer + 1 < input.len()
            && (input[op_pointer + 1] == b' ' || input[op_pointer + 1] == b'\n')
        {
            let num_bytes = (0..rows - 1)
                .map(|i| input[cols * i + (op_pointer - ops_offset)])
                .filter(|c| *c != b' ')
                .collect::<Vec<u8>>();
            let num = usize_from_bytes(&num_bytes);
            match op {
                b'*' => cur_res *= num,
                b'+' => cur_res += num,
                _ => panic!(),
            };
            op_pointer += 1;
        } else {
            res += cur_res;
            op_pointer += 1;
            if op_pointer >= input.len() {
                break;
            }
            op = input[op_pointer];
            cur_res = match op {
                b'*' => 1,
                b'+' => 0,
                _ => panic!("{}", (op_pointer - ops_offset)),
            };
        }
    }

    res
}

pub fn main() {
    let input = INPUT;

    println!("{}", part1(input));
    println!("{}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/day06.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT;
        assert_eq!(part1(input), 4277556);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT;
        assert_eq!(part2(input), 3263827);
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = INPUT;
        b.iter(|| part1(input))
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = INPUT;
        b.iter(|| part2(input))
    }
}
