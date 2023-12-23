use core::panic;
use regex::bytes::Regex;
use std::fs::read_to_string;
fn str_to_int(capture: &[u8]) -> Option<u8> {
    match capture {
        b"one" | b"1" => Some(1),
        b"two" | b"2" => Some(2),
        b"three" | b"3" => Some(3),
        b"four" | b"4" => Some(4),
        b"five" | b"5" => Some(5),
        b"six" | b"6" => Some(6),
        b"seven" | b"7" => Some(7),
        b"eight" | b"8" => Some(8),
        b"nine" | b"9" => Some(9),
        b"zero" | b"0" => Some(0),
        b"\n" => None,
        _ => panic!(),
    }
}

pub fn solve_day() -> u32 {
    solve_file(read_to_string("inputs/day1.txt").unwrap())
}
fn solve_file(text: String) -> u32 {
    let mut text = text.as_bytes();
    let mut sum: u32 = 0;
    let re =
        Regex::new(r"one|two|three|four|five|six|seven|eight|nine|zero|0|1|2|3|4|5|6|7|8|9|\n")
            .unwrap();
    let mut num = 0u8;
    let mut new_line = true;
    while let Some(capture) = re.find(text) {
        if let Some(val) = str_to_int(capture.as_bytes()) {
            num = val;
            if new_line {
                sum += num as u32 * 10;
                new_line = !new_line;
            }
        } else {
            // new line, add last num to sum as second digit
            new_line = true;
            sum += num as u32;
        }
        text = if capture.len() == 1 {
            &text[capture.end()..]
        } else {
            &text[capture.end() - 1..]
        };
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        assert_eq!(
            solve_file(read_to_string("inputs/day1b_test.txt").unwrap()),
            281
        );
        assert_eq!(
            solve_file(read_to_string("inputs/day1.txt").unwrap()),
            55291
        );
    }
}
