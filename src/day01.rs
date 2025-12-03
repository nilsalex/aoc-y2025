extern crate test;

const INPUT: &[u8] = include_bytes!("../inputs/day01.txt");

const POWERS_OF_TEN: [i32; 6] = [1, 10, 100, 1000, 10000, 100000];

fn i32_from_bytes(bytes: &[u8]) -> i32 {
    bytes.iter().rev().enumerate().fold(0, |acc, (ix, x)| {
        acc + (x & 0x0f) as i32 * POWERS_OF_TEN[ix]
    })
}

pub fn part1(input: &[u8]) -> usize {
    input
        .split(|c| *c == b'\n')
        .map(|line| {
            let sign = if line[0] == b'L' { -1 } else { 1 };
            let x = i32_from_bytes(&line[1..]);
            sign * x
        })
        .scan(50, |state, x| {
            *state += x;
            Some(*state)
        })
        .filter(|&x| x.rem_euclid(100) == 0)
        .count()
}

pub fn part2(input: &[u8]) -> usize {
    input
        .split(|c| *c == b'\n')
        .map(|line| {
            let sign = if line[0] == b'L' { -1 } else { 1 };
            let x = i32_from_bytes(&line[1..]);
            sign * x
        })
        .fold((1000050, 0), |state, x| {
            let last_pos = state.0;
            let last_count = state.1;
            let correction = if x < 0 { -1 } else { 0 };
            let next_pos = last_pos + x;
            let next_count =
                last_count + count_100_between(last_pos + correction, next_pos + correction);
            (next_pos, next_count)
        })
        .1
}

fn count_100_between(start: i32, end: i32) -> usize {
    let start_hundreds = start / 100;
    let end_hundreds = end / 100;
    start_hundreds.abs_diff(end_hundreds) as usize
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

    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/day01.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part1(input), 3);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part2(input), 6);
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
