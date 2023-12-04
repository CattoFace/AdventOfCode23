use std::{fs::File, io::BufRead, io::BufReader};

pub fn solve_day() -> u32 {
    let file = File::open("inputs/day1.txt").unwrap();
    let ans = solve_file(file);
    assert_eq!(ans, 55607);
    ans
}
fn solve_file(file: File) -> u32 {
    let lines = BufReader::new(file).lines();
    lines
        .map(|line| {
            let unwrapped_line = line.unwrap();
            let first = unwrapped_line.chars().find_map(|x| x.to_digit(10)).unwrap();
            let last = unwrapped_line
                .chars()
                .rev()
                .find_map(|x| x.to_digit(10))
                .unwrap();
            first * 10 + last
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        assert_eq!(
            solve_file(File::open("inputs/day1a_test.txt").unwrap()),
            142
        );
        assert_eq!(solve_file(File::open("inputs/day1.txt").unwrap()), 55607)
    }
}
