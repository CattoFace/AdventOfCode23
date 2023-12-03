use core::panic;
use regex::Regex;
use std::{fs::File, io::BufRead, io::BufReader};

fn str_to_int(capture: &str) -> u32 {
    match capture {
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine" | "9" => 9,
        "zero" | "0" => 0,
        _ => panic!(),
    }
}
fn str_to_int_rev(capture: &str) -> u32 {
    match capture {
        "eno" | "1" => 1,
        "owt" | "2" => 2,
        "eerht" | "3" => 3,
        "ruof" | "4" => 4,
        "evif" | "5" => 5,
        "xis" | "6" => 6,
        "neves" | "7" => 7,
        "thgie" | "8" => 8,
        "enin" | "9" => 9,
        "orez" | "0" => 0,
        _ => panic!(),
    }
}

pub fn solve_day() {
    let file = File::open("inputs/day1.txt").unwrap();
    assert_eq!(solve_file(file), 55291);
    println!("pass")
}
fn solve_file(file: File) -> u32 {
    println!("Solving day1b");
    let lines = BufReader::new(file).lines();
    let mut sum: u32 = 0;
    let re_first = Regex::new(
        r".*?(?<first>one|two|three|four|five|six|seven|eight|nine|zero|0|1|2|3|4|5|6|7|8|9)",
    )
    .unwrap();
    let re_last = Regex::new(
        r".*?(?<last>orez|enin|thgie|neves|xis|evif|ruof|eerht|owt|eno|0|1|2|3|4|5|6|7|8|9)",
    )
    .unwrap();
    for line in lines {
        let unwrapped_line = line.unwrap();
        let rev_line = unwrapped_line.chars().rev().collect::<String>();
        let captures_first = re_first.captures(unwrapped_line.as_str()).unwrap();
        let captures_last = re_last.captures(rev_line.as_str()).unwrap();
        sum += 10 * str_to_int(&captures_first["first"]) + str_to_int_rev(&captures_last["last"]);
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
            solve_file(File::open("inputs/day1b_test.txt").unwrap()),
            281
        );
        assert_eq!(solve_file(File::open("inputs/day1.txt").unwrap()), 55291)
    }
}
