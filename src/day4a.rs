use std::{collections::HashSet, fs::File, io::BufRead, io::BufReader};

pub fn solve_day() -> u32 {
    let file = File::open("inputs/day4.txt").unwrap();
    let ans = solve_file(file);
    assert_eq!(ans, 19855);
    ans
}
fn solve_file(file: File) -> u32 {
    let lines = BufReader::new(file).lines();
    lines
        .map(|line| {
            let unwrapped_line = line.unwrap();
            let (_line_start, game) = unwrapped_line.split_once(':').unwrap();
            let (winning, available) = game.split_once('|').unwrap();
            let winning_nums: HashSet<u32> = winning
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect();
            let matching_count = available
                .split_whitespace()
                .filter(|s| winning_nums.contains(&s.parse::<u32>().unwrap()))
                .count();
            if matching_count == 0 {
                0
            } else {
                2u32.pow((matching_count - 1) as u32)
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        assert_eq!(solve_file(File::open("inputs/day4_test.txt").unwrap()), 13);
        assert_eq!(solve_file(File::open("inputs/day4.txt").unwrap()), 19855);
    }
}
