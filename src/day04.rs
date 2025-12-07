extern crate test;

const INPUT: &[u8] = include_bytes!("../inputs/day04.txt");

pub fn part1(input: &[u8]) -> usize {
    let cols = input.iter().take_while(|c| **c != b'\n').count();

    let mut grid = vec![vec![b'.'; cols + 2]];

    for line in input.split(|c| *c == b'\n') {
        grid.push(
            std::iter::once(b'.')
                .chain(line.iter().cloned())
                .chain(std::iter::once(b'.'))
                .collect(),
        );
    }

    grid.push(vec![b'.'; cols + 2]);

    let mut result = 0;

    for col in 1..=cols {
        for row in 1..=grid.len() - 2 {
            if grid[col][row] != b'@' {
                continue;
            }

            let adj_roll_count = [
                grid[col - 1][row - 1],
                grid[col][row - 1],
                grid[col + 1][row - 1],
                grid[col - 1][row],
                grid[col + 1][row],
                grid[col - 1][row + 1],
                grid[col][row + 1],
                grid[col + 1][row + 1],
            ]
            .into_iter()
            .filter(|c| *c == b'@')
            .count();

            if adj_roll_count < 4 {
                result += 1;
            }
        }
    }

    result
}

pub fn part2(input: &[u8]) -> usize {
    let cols = input.iter().take_while(|c| **c != b'\n').count();

    let mut grid = vec![vec![b'.'; cols + 2]];

    for line in input.split(|c| *c == b'\n') {
        grid.push(
            std::iter::once(b'.')
                .chain(line.iter().cloned())
                .chain(std::iter::once(b'.'))
                .collect(),
        );
    }

    grid.push(vec![b'.'; cols + 2]);

    let mut result = 0;

    loop {
        let mut removed = false;
        for col in 1..=cols {
            for row in 1..=grid.len() - 2 {
                if grid[col][row] != b'@' {
                    continue;
                }

                let adj_roll_count = [
                    grid[col - 1][row - 1],
                    grid[col][row - 1],
                    grid[col + 1][row - 1],
                    grid[col - 1][row],
                    grid[col + 1][row],
                    grid[col - 1][row + 1],
                    grid[col][row + 1],
                    grid[col + 1][row + 1],
                ]
                .into_iter()
                .filter(|c| *c == b'@')
                .count();

                if adj_roll_count < 4 {
                    grid[col][row] = b'.';
                    removed = true;
                    result += 1;
                }
            }
        }
        if !removed {
            break;
        }
    }

    result
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

    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/day04.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part1(input), 13);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part2(input), 43);
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
