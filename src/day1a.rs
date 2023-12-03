use std::{fs::File, io::BufRead, io::BufReader};

pub fn solve_day() {
    let file = File::open("inputs/day1.txt").unwrap();
    assert_eq!(solve_file(file), 55607);
    println!("pass")
}
fn solve_file(file: File) -> u32 {
    println!("Solving day1a");
    let lines = BufReader::new(file).lines();
    let mut sum: u32 = 0;
    for line in lines {
        let unwrapped_line = line.unwrap();
        let first = unwrapped_line.chars().find_map(|x| x.to_digit(10)).unwrap();
        let last = unwrapped_line
            .chars()
            .rev()
            .find_map(|x| x.to_digit(10))
            .unwrap();
        sum += first * 10 + last;
    }
    println!("{}", sum);
    sum
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
