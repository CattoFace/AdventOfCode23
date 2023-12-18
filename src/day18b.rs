use std::{
    fs::read_to_string,
    iter::{self, once},
    str,
};

use itertools::Itertools;
pub fn solve_day() -> i64 {
    solve_file(read_to_string("inputs/day18.txt").unwrap())
}
fn solve_file(text: String) -> i64 {
    let base_coord = 0i32;
    let text = text.as_bytes();
    let mut x = base_coord / 2;
    let mut y = base_coord / 2;
    let mut perimeter_points = 0i32;
    let mut index = 0usize;
    let inner_area = once((x, y))
        .chain(iter::from_fn(|| {
            if index == text.len() {
                return None;
            }
            // add based on whether the original length is 1 or 2 digits
            index += if text[index + 4] == b' ' { 7 } else { 6 };
            let length = i32::from_str_radix(
                unsafe { str::from_utf8_unchecked(&text[index..index + 5]) },
                16,
            )
            .unwrap();
            perimeter_points += length;
            match text[index + 5] {
                // R
                b'0' => {
                    x += length;
                }
                // D
                b'1' => {
                    y += length;
                }
                // L
                b'2' => {
                    x -= length;
                }
                // U
                b'3' => {
                    y -= length;
                }
                _ => unreachable!(),
            }
            // skip to next line
            index += 8;
            Some((x, y))
        }))
        .tuple_windows()
        .map(|((x1, y1), (x2, y2))| x1 as i64 * y2 as i64 - x2 as i64 * y1 as i64)
        .sum::<i64>()
        / 2;
    perimeter_points as i64 / 2 + 1 + inner_area
    // print_map(&map, size, x_min, x_max, y_min, y_max);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        assert_eq!(
            solve_file(read_to_string("inputs/day18_test.txt").unwrap()),
            952408144115
        );
        assert_eq!(
            solve_file(read_to_string("inputs/day18.txt").unwrap()),
            173152345887206
        );
    }
}
