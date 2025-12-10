extern crate test;

use std::collections::HashSet;

const INPUT: &[u8] = include_bytes!("../inputs/day08.txt");

const POWERS_OF_TEN: [usize; 8] = [1, 10, 100, 1000, 10000, 100000, 1000000, 10000000];

fn usize_from_bytes(bytes: &[u8]) -> usize {
    bytes.iter().rev().enumerate().fold(0, |acc, (ix, x)| {
        acc + (x & 0x0f) as usize * POWERS_OF_TEN[ix]
    })
}

pub fn part1(num_pairs: usize, input: &[u8]) -> usize {
    let boxes = input
        .split(|c| *c == b'\n')
        .map(|line| {
            let mut it = line.split(|c| *c == b',');
            let x = usize_from_bytes(it.next().unwrap());
            let y = usize_from_bytes(it.next().unwrap());
            let z = usize_from_bytes(it.next().unwrap());
            (x, y, z)
        })
        .collect::<Vec<_>>();

    let mut sets = (0..boxes.len())
        .map(|b| HashSet::from([b]))
        .collect::<Vec<_>>();

    let mut dists = vec![];

    for i in 0..boxes.len() - 1 {
        let (ix, iy, iz) = boxes[i];
        for (j, &bj) in boxes.iter().enumerate().skip(i + 1) {
            let (jx, jy, jz) = bj;
            dists.push((
                i,
                j,
                ix.abs_diff(jx).pow(2) + iy.abs_diff(jy).pow(2) + iz.abs_diff(jz).pow(2),
            ));
        }
    }

    dists.sort_by_key(|(_, _, d)| *d);

    for (i, j, _) in dists.into_iter().take(num_pairs) {
        let i_set = sets.iter().position(|s| s.contains(&i)).unwrap();
        let j_set = sets.iter().position(|s| s.contains(&j)).unwrap();

        if i_set == j_set {
            continue;
        }

        let mut next_sets: Vec<HashSet<usize>> = vec![];

        for (k, s) in sets.iter().enumerate() {
            if k == i_set || k == j_set {
                continue;
            }
            next_sets.push(s.clone());
        }

        let union: HashSet<usize> = sets[i_set].union(&sets[j_set]).cloned().collect();

        next_sets.push(union);

        std::mem::swap(&mut sets, &mut next_sets);
    }

    let mut counts = sets.iter().map(|s| s.len()).collect::<Vec<_>>();
    counts.sort();
    counts.reverse();

    counts[0] * counts[1] * counts[2]
}

pub fn part2(input: &[u8]) -> usize {
    let boxes = input
        .split(|c| *c == b'\n')
        .map(|line| {
            let mut it = line.split(|c| *c == b',');
            let x = usize_from_bytes(it.next().unwrap());
            let y = usize_from_bytes(it.next().unwrap());
            let z = usize_from_bytes(it.next().unwrap());
            (x, y, z)
        })
        .collect::<Vec<_>>();

    let mut sets = (0..boxes.len())
        .map(|b| HashSet::from([b]))
        .collect::<Vec<_>>();

    let mut dists = vec![];

    for i in 0..boxes.len() - 1 {
        let (ix, iy, iz) = boxes[i];
        for (j, &bj) in boxes.iter().enumerate().skip(i + 1) {
            let (jx, jy, jz) = bj;
            dists.push((
                i,
                j,
                ix.abs_diff(jx).pow(2) + iy.abs_diff(jy).pow(2) + iz.abs_diff(jz).pow(2),
            ));
        }
    }

    dists.sort_by_key(|(_, _, d)| *d);

    for (i, j, _) in dists {
        let i_set = sets.iter().position(|s| s.contains(&i)).unwrap();
        let j_set = sets.iter().position(|s| s.contains(&j)).unwrap();

        if i_set == j_set {
            continue;
        }

        let mut next_sets: Vec<HashSet<usize>> = vec![];

        for (k, s) in sets.iter().enumerate() {
            if k == i_set || k == j_set {
                continue;
            }
            next_sets.push(s.clone());
        }

        let union: HashSet<usize> = sets[i_set].union(&sets[j_set]).cloned().collect();

        next_sets.push(union);

        std::mem::swap(&mut sets, &mut next_sets);

        if sets.len() == 1 {
            return boxes[i].0 * boxes[j].0;
        }
    }

    panic!();
}

pub fn main() {
    let input = INPUT.trim_ascii_end();

    println!("{}", part1(1000, input));
    println!("{}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    const TEST_INPUT: &[u8] = include_bytes!("../test_inputs/day08.txt");

    #[test]
    fn test_part1() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part1(10, input), 40);
    }

    #[test]
    fn test_part2() {
        let input = TEST_INPUT.trim_ascii_end();
        assert_eq!(part2(input), 25272);
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = INPUT.trim_ascii_end();
        b.iter(|| part1(1000, input))
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = INPUT.trim_ascii_end();
        b.iter(|| part2(input))
    }
}
