extern crate test;

const INPUT: &[u8] = include_bytes!("../inputs/day03.txt");

pub fn part1(input: &[u8]) -> usize {
    input
        .split(|c| *c == b'\n')
        .map(|line| {
            let nums = line
                .iter()
                .map(|c| (c - b'0') as usize)
                .collect::<Vec<usize>>();

            let mut pivot = 0;
            let mut pivot_pos = 0;
            for p in (1..=9_usize).rev() {
                if let Some(pos) = nums.iter().position(|x| *x == p)
                    && pos + 1 < nums.len()
                {
                    pivot = p;
                    pivot_pos = pos;
                    break;
                }
            }

            let next = nums.iter().skip(pivot_pos + 1).max().unwrap();

            10 * pivot + next
        })
        .sum()
}

pub fn part2(input: &[u8]) -> usize {
    input
        .split(|c| *c == b'\n')
        .map(|line| {
            let nums = line
                .iter()
                .map(|c| (c - b'0') as usize)
                .collect::<Vec<usize>>();

            let mut results = [0; 12];
            let mut earliest_pivot_pos = 0;

            for (i, r) in results.iter_mut().enumerate() {
                let mut found = false;
                for p in (1..=9_usize).rev() {
                    if let Some((pos, _)) = nums
                        .iter()
                        .enumerate()
                        .skip(earliest_pivot_pos)
                        .find(|(_, x)| **x == p)
                        && pos + (11 - i) < nums.len()
                    {
                        *r = p;
                        earliest_pivot_pos = pos + 1;
                        found = true;
                        break;
                    }
                }
                assert!(found);
            }

            results.iter().fold(0, |acc, x| acc * 10 + x)
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

    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/day03.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part1(input), 357);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part2(input), 3121910778619);
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
