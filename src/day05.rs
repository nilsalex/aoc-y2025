extern crate test;

const INPUT: &[u8] = include_bytes!("../inputs/day05.txt");

const POWERS_OF_TEN: [usize; 20] = [
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
    1000000000000,
    10000000000000,
    100000000000000,
    1000000000000000,
    10000000000000000,
    100000000000000000,
    1000000000000000000,
    10000000000000000000,
];

fn usize_from_bytes(bytes: &[u8]) -> usize {
    bytes.iter().rev().enumerate().fold(0, |acc, (ix, x)| {
        acc + (x & 0x0f) as usize * POWERS_OF_TEN[ix]
    })
}

pub fn part1(input: &[u8]) -> usize {
    let mut lines = input.split(|c| *c == b'\n');
    let ranges = lines
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|line| {
            let mut it = line.split(|b| *b == b'-');
            let start = usize_from_bytes(it.next().unwrap());
            let end = usize_from_bytes(it.next().unwrap());
            assert!(format!("{}-{}", start, end) == std::str::from_utf8(line).unwrap());
            assert!(start <= end);
            (start, end)
        })
        .collect::<Vec<(usize, usize)>>();

    lines
        .map(|line| {
            let x = usize_from_bytes(line);
            assert!(format!("{}", x) == std::str::from_utf8(line).unwrap());
            x
        })
        .filter(|x| ranges.iter().any(|&(s, e)| s <= *x && *x <= e))
        .count()
}

pub fn part2(input: &[u8]) -> usize {
    let mut lines = input.split(|c| *c == b'\n');
    let mut ranges = lines
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|line| {
            let mut it = line.split(|b| *b == b'-');
            let start = usize_from_bytes(it.next().unwrap());
            let end = usize_from_bytes(it.next().unwrap());
            assert!(format!("{}-{}", start, end) == std::str::from_utf8(line).unwrap());
            assert!(start <= end);
            (start, end)
        })
        .collect::<Vec<(usize, usize)>>();

    ranges.sort();

    let (final_start, final_end, sum) = ranges.iter().fold(
        (ranges[0].0, ranges[0].1, 0),
        |(acc_start, acc_end, acc), (s, e)| {
            if *s > acc_end {
                (*s, *e, acc + acc_end - acc_start + 1)
            } else {
                (
                    std::cmp::min(acc_start, *s),
                    std::cmp::max(acc_end, *e),
                    acc,
                )
            }
        },
    );

    sum + final_end - final_start + 1
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

    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/day05.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part1(input), 3);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part2(input), 14);
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
