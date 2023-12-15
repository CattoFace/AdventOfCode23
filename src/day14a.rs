use std::fs::read_to_string;

pub fn solve_day() -> u32 {
    solve_file(read_to_string("inputs/day14_test2.txt").unwrap())
}
pub fn solve_file(text: String) -> u32 {
    let width = text.find('\n').unwrap() + 1;
    let height = (text.len() + 1) / width;
    let text = text.as_bytes();
    let mut sum = 0u32;
    for col_index in 0..(width - 1) {
        let mut next_spot = 0u32;
        for row_index in 0..height {
            match text[row_index * width + col_index] {
                b'#' => {
                    next_spot = row_index as u32 + 1;
                }
                b'O' => {
                    sum += height as u32 - next_spot;
                    next_spot += 1;
                }
                b'.' => {}
                _ => unreachable!(),
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        assert_eq!(
            solve_file(read_to_string("inputs/day14_test.txt").unwrap()),
            136
        );
        assert_eq!(
            solve_file(read_to_string("inputs/day14.txt").unwrap()),
            113486
        );
    }
}
