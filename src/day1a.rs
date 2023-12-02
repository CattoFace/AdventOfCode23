use std::{fs::File, io::BufRead, io::BufReader};

pub fn main() {
    let file = File::open("inputs/day1.txt").unwrap();
    let lines = BufReader::new(file).lines();
    let mut sum: u32 = 0;
    for line in lines {
        let unwrapped_line = line.unwrap();
        let first = unwrapped_line.chars().find(|x| x.is_ascii_digit()).unwrap();
        let last = unwrapped_line
            .chars()
            .rev()
            .find(|x| x.is_ascii_digit())
            .unwrap();
        sum += first.to_digit(10).unwrap() * 10 + last.to_digit(10).unwrap();
    }
    println!("{}", sum);
}
