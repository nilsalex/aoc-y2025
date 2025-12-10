extern crate test;

const INPUT: &[u8] = include_bytes!("../inputs/day07.txt");

fn solve(input: &[u8]) -> (usize, usize) {
    let mut grid = input
        .split(|c| *c == b'\n')
        .map(|line| {
            line.iter()
                .map(|c| {
                    (
                        *c,
                        match *c {
                            b'S' => 1,
                            _ => 0,
                        },
                    )
                })
                .collect::<Vec<(u8, usize)>>()
        })
        .collect::<Vec<_>>();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut splits = 0;

    for row in 1..rows {
        for col in 0..cols {
            let center = grid[row - 1][col];
            if grid[row][col].0 == b'^' && grid[row - 1][col].0 == b'|' {
                splits += 1;
            }
            if grid[row][col].0 != b'^' && (center.0 == b'S' || center.0 == b'|') {
                grid[row][col].0 = b'|';
                grid[row][col].1 += center.1;
            }
            if col + 1 < cols && grid[row][col + 1].0 == b'^' && grid[row - 1][col + 1].0 == b'|' {
                grid[row][col].0 = b'|';
                grid[row][col].1 += grid[row - 1][col + 1].1;
            }
            if col > 0 && grid[row][col - 1].0 == b'^' && grid[row - 1][col - 1].0 == b'|' {
                grid[row][col].0 = b'|';
                grid[row][col].1 += grid[row - 1][col - 1].1;
            }
        }
    }

    (splits, grid.last().unwrap().iter().map(|(_, c)| c).sum())
}

pub fn part1(input: &[u8]) -> usize {
    solve(input).0
}

pub fn part2(input: &[u8]) -> usize {
    solve(input).1
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

    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/day07.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part1(input), 21);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part2(input), 40);
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
