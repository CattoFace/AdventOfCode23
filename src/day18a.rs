use std::{
    fs::read_to_string,
    iter::{self, once},
    str,
};

use itertools::Itertools;
pub fn solve_day() -> u32 {
    solve_file(read_to_string("inputs/day18.txt").unwrap())
}

fn solve_file(text: String) -> u32 {
    let base_coord = 0i32;
    let text = text.as_bytes();
    let mut x = base_coord / 2;
    let mut y = base_coord / 2;
    let mut perimeter_points = 0u32;
    let mut index = 0usize;
    let inner_area = once((x, y))
        .chain(iter::from_fn(|| {
            if index == text.len() {
                return None;
            }
            let num_end = (text[index + 2..].iter().position(|&c| c == b' ')).unwrap();
            let length = unsafe {
                str::from_utf8_unchecked(&text[index + 2..index + 2 + num_end])
                    .parse::<i32>()
                    .unwrap()
            };
            perimeter_points += length as u32;
            match text[index] {
                // R
                b'R' => {
                    x += length;
                }
                // D
                b'D' => {
                    y += length;
                }
                // L
                b'L' => {
                    x -= length;
                }
                // U
                b'U' => {
                    y -= length;
                }
                _ => unreachable!(),
            }
            index += 13 + num_end; // jump to next line, account for size of length
            Some((x, y))
        }))
        .tuple_windows()
        .map(|((x1, y1), (x2, y2))| x1 * y2 - x2 * y1)
        .sum::<i32>()
        / 2;
    perimeter_points / 2 + 1 + (inner_area as u32)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        assert_eq!(
            solve_file(read_to_string("inputs/day18_test.txt").unwrap()),
            62
        );
        assert_eq!(
            solve_file(read_to_string("inputs/day18.txt").unwrap()),
            47139
        );
    }
}
