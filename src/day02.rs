extern crate test;

use std::collections::HashSet;

const INPUT: &[u8] = include_bytes!("../inputs/day02.txt");

const POWERS_OF_TEN: [usize; 12] = [
    1,
    10,
    100,
    1000,
    10000,
    100000,
    1000000,
    10000000,
    100000000,
    1000000000,
    10000000000,
    100000000000,
];

fn usize_from_bytes(bytes: &[u8]) -> usize {
    bytes.iter().rev().enumerate().fold(0, |acc, (ix, x)| {
        acc + (x & 0x0f) as usize * POWERS_OF_TEN[ix]
    })
}

pub fn part1(input: &[u8]) -> usize {
    input
        .split(|c| *c == b',')
        .filter_map(|line| {
            let mut it = line.split(|c| *c == b'-');
            let start = it.next().unwrap();
            let end = it.next().unwrap();

            let start_len = start.len();
            let end_len = end.len();

            match (start_len % 2, end_len % 2) {
                (0, 0) => Some((start.to_vec(), end.to_vec())),
                (1, 1) => None,
                (1, 0) => {
                    let mut start_vec = vec![0; end_len];
                    start_vec[0] = b'1';
                    Some((start_vec, end.to_vec()))
                }
                (0, 1) => {
                    let end_vec = vec![9; start_len];
                    Some((start.to_vec(), end_vec))
                }
                _ => None,
            }
        })
        .map(|(start_vec, end_vec)| {
            let mut result = 0;
            let half_len = start_vec.len() / 2;
            let fac = 10_usize.pow(half_len as u32) + 1;

            let start = usize_from_bytes(&start_vec);
            let end = usize_from_bytes(&end_vec);

            let start_half = usize_from_bytes(&start_vec[0..half_len]);
            let end_half = usize_from_bytes(&end_vec[0..half_len]);

            for i in start_half..=end_half {
                let x = fac * i;
                if x >= start && x <= end {
                    result += x;
                }
            }

            result
        })
        .sum()
}

pub fn part2(input: &[u8]) -> usize {
    input
        .split(|c| *c == b',')
        .flat_map(|line| {
            let mut it = line.split(|c| *c == b'-');
            let start = it.next().unwrap();
            let end = it.next().unwrap();

            let start_len = start.len();
            let end_len = end.len();

            if start_len == end_len {
                vec![(start.to_vec(), end.to_vec())]
            } else {
                let end_vec = vec![9; start_len];
                let mut start_vec = vec![0; end_len];
                start_vec[0] = b'1';

                vec![(start.to_vec(), end_vec), (start_vec, end.to_vec())]
            }
        })
        .map(|(start_vec, end_vec)| {
            let mut seen = HashSet::new();

            let mut result = 0;

            let start = usize_from_bytes(&start_vec);
            let end = usize_from_bytes(&end_vec);

            let len = start_vec.len();

            for i in 1..=len / 2 {
                let reps = len / i;
                if reps * i != len {
                    continue;
                }

                let base_fac = 10_usize.pow(i as u32);

                let mut fac = 1;

                for _ in 1..reps {
                    fac = fac * base_fac + 1;
                }

                let start_seg = usize_from_bytes(&start_vec[0..i]);
                let end_seg = usize_from_bytes(&end_vec[0..i]);

                for j in start_seg..=end_seg {
                    let x = fac * j;
                    if x >= start && x <= end && !seen.contains(&x) {
                        result += x;
                        seen.insert(x);
                    }
                }
            }

            result
        })
        .sum()
}

pub fn main() {
    let input = INPUT.trim_ascii_end();

    println!("{}", part1(input));
    println!("{}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/day02.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part1(input), 1227775554);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part2(input), 4174379265);
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = INPUT.trim_ascii_end();
        b.iter(|| part1(input))
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = INPUT.trim_ascii_end();
        b.iter(|| part2(input))
    }
}
