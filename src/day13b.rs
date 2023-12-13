use std::fs::read_to_string;

use itertools::Itertools;
use rayon::prelude::{ParallelBridge, ParallelIterator};

pub fn solve_day() -> u32 {
    solve_file(read_to_string("inputs/day13.txt").unwrap())
}

fn split_equal(line: &str, pos: usize) -> (&str, &str) {
    let size = pos.min(line.len() - pos);
    (&line[(pos - size)..pos], &line[pos..(pos + size)])
}
fn process_pattern(pattern: &str) -> u32 {
    if pattern.is_empty() {
        return 0;
    }
    let lines = pattern.lines().collect_vec();
    if pattern.is_empty() {
        return 0;
    }
    let width = pattern.find('\n').unwrap();
    let height = pattern.len() / (width + 1) + 1;
    // check every possible column mirror
    if let Some(col_mirror) = (1..width).position(|index| {
        // check every line matches the column mirror
        lines
            .iter()
            .flat_map(|l| {
                let (left, right) = split_equal(l, index);
                let ans = left
                    .chars()
                    .zip(right.chars().rev())
                    .filter(|(a, b)| a != b);
                ans
            })
            .take(2)
            .count()
            == 1usize
    }) {
        return col_mirror as u32 + 1;
    }
    ((1..height)
        .position(|index| {
            let size = index.min(height - index);
            ((index - size)..index)
                .zip((index..(index + size)).rev())
                .flat_map(|(l1, l2)| {
                    let left = lines[l1];
                    let right = lines[l2];
                    left.chars().zip(right.chars()).filter(|(a, b)| a != b)
                })
                .take(2)
                .count()
                == 1usize
        })
        .unwrap() as u32
        + 1)
        * 100
}

fn solve_file(text: String) -> u32 {
    text.split("\n\n").par_bridge().map(process_pattern).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        assert_eq!(
            solve_file(read_to_string("inputs/day13_test.txt").unwrap()),
            400
        );
        assert_eq!(
            solve_file(read_to_string("inputs/day13.txt").unwrap()),
            30449
        );
    }
}
