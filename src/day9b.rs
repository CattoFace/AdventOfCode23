use std::{fs::File, io::BufRead, io::BufReader};

use itertools::Itertools;

pub fn solve_day() -> i32 {
    let file = File::open("inputs/day9.txt").unwrap();
    solve_file(file)
}
fn solve_file(file: File) -> i32 {
    let lines = BufReader::new(file).lines();
    lines.map(|l| process_line(&l.unwrap())).sum()
}

fn process_line(line: &str) -> i32 {
    let mut nums = line
        .split_ascii_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect_vec();
    let mut done = false;
    let mut first_pos = 1usize;
    // println!("{:?}", nums);
    while !done {
        done = true;
        for i in (first_pos..nums.len()).rev() {
            // print!("{},", i);
            nums[i] -= nums[i - 1];
            if nums[i] != 0 {
                done = false;
            }
        }
        first_pos += 1;
        // println!("{:?}", nums);
    }
    let mut ans = 0i32;
    for i in (0..first_pos).rev() {
        ans = nums[i] - ans;
    }
    // println!("{}", ans);
    ans
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        assert_eq!(solve_file(File::open("inputs/day9_test.txt").unwrap()), 2);
        assert_eq!(solve_file(File::open("inputs/day9.txt").unwrap()), 900);
    }
}
