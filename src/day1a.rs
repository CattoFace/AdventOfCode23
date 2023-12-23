use std::fs::read_to_string;
pub fn solve_day() -> u32 {
    solve_file(read_to_string("inputs/day1.txt").unwrap())
}
fn solve_file(text: String) -> u32 {
    let mut text = text.as_bytes();
    let mut sum = 0u32;
    loop {
        while !text[0].is_ascii_digit() {
            text = &text[1..];
        }
        let mut num = text[0] - b'0';
        sum += num as u32 * 10;
        while text[0] != b'\n' {
            text = &text[1..];
            if text[0].is_ascii_digit() {
                num = text[0] - b'0';
            }
        }
        sum += num as u32;
        text = &text[1..];
        if text.is_empty() {
            return sum;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        assert_eq!(
            solve_file(read_to_string("inputs/day1a_test.txt").unwrap()),
            142
        );
        assert_eq!(
            solve_file(read_to_string("inputs/day1.txt").unwrap()),
            55607
        )
    }
}
