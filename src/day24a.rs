use std::{fs::read_to_string, ops::Range};

use rayon::{
    current_num_threads,
    prelude::{ParallelBridge, ParallelIterator},
};

#[derive(Debug)]
struct Line2D {
    m: f32,
    n: f32,
    start_y: f32,
    y_rising: bool,
}
impl Line2D {
    fn intersection_within(&self, other: &Line2D, range: &Range<f32>) -> bool {
        if self.m == other.m {
            return false;
        }
        let x_intersection = (other.n - self.n) / (self.m - other.m);
        let y_intersection = self.m * x_intersection + self.n;
        ((y_intersection < self.start_y) ^ (self.y_rising))
            && ((y_intersection < other.start_y) ^ (other.y_rising))
            && range.contains(&x_intersection)
            && range.contains(&y_intersection)
    }
}
fn atoi_neg(text: &[u8]) -> (i64, usize) {
    let mut sum = 0i64;
    let mut offset = 0usize;
    let neg = if text[0] == b'-' {
        offset += 1;
        true
    } else {
        false
    };
    while text[offset].is_ascii_digit() {
        sum = sum * 10 + (text[offset] - b'0') as i64;
        offset += 1;
    }
    (if neg { -sum } else { sum }, offset)
}
fn atoi(text: &[u8]) -> (i64, usize) {
    let mut sum = 0i64;
    let mut offset = 0usize;
    while text[offset].is_ascii_digit() {
        sum = sum * 10 + (text[offset] - b'0') as i64;
        offset += 1;
    }
    (sum, offset)
}
fn parse_text(text: String) -> Vec<Line2D> {
    let mut text = text.as_bytes();
    let mut lines = Vec::<Line2D>::new();
    loop {
        if text.is_empty() {
            break;
        }
        let (start_x, offset) = atoi(text);
        text = &text[(offset + 2)..];
        let (start_y, offset) = atoi(text);
        text = &text[(offset + 2)..];
        let offset = text.iter().position(|&c| c == b'@').unwrap() + 2;
        text = &text[offset..];
        let (velocity_x, offset) = atoi_neg(text);
        text = &text[(offset + 2)..];
        let (velocity_y, offset) = atoi_neg(text);
        text = &text[(offset + 2)..];
        let line_end = text.iter().position(|&c| c == b'\n').unwrap() + 1;
        text = &text[line_end..];
        let m = velocity_y as f32 / velocity_x as f32;
        let n = start_y as f32 - m * start_x as f32;
        lines.push(Line2D {
            m,
            n,
            start_y: start_y as f32,
            y_rising: velocity_y > 0,
        });
    }
    lines
}
fn count_intersections(lines: &[Line2D], range: Range<f32>) -> usize {
    (0..current_num_threads())
        .par_bridge()
        .map(|thread| {
            (thread..lines.len())
                .step_by(current_num_threads())
                .map(|i| {
                    lines[i + 1..]
                        .iter()
                        .filter(|&line| line.intersection_within(&lines[i], &range))
                        .count()
                })
                .sum::<usize>()
        })
        .sum()
}
pub fn solve_day() -> usize {
    // solve_file(
    //     read_to_string("inputs/day24_test.txt").unwrap(),
    //     7f32..27f32,
    // )
    solve_file(
        read_to_string("inputs/day24.txt").unwrap(),
        200000000000000f32..400000000000000f32,
    )
}
fn solve_file(text: String, range: Range<f32>) -> usize {
    let lines = parse_text(text);
    // println!("{:?}", lines);
    count_intersections(&lines, range)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        assert_eq!(
            solve_file(
                read_to_string("inputs/day24_test.txt").unwrap(),
                7f32..27f32
            ),
            2
        );
        assert_eq!(
            solve_file(
                read_to_string("inputs/day24.txt").unwrap(),
                200000000000000f32..400000000000000f32
            ),
            11246
        );
    }
}
