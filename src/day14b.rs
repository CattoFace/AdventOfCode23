use std::fs::read_to_string;

use itertools::Itertools;

pub fn solve_day() -> u32 {
    solve_file(read_to_string("inputs/day14.txt").unwrap())
}

fn slide_east(text: &mut [u8], width: usize, height: usize) {
    for row_index in 0..height {
        let mut next_spot = width - 1;
        for col_index in (0..(width - 1)).rev() {
            match text[row_index * width + col_index] {
                b'#' => {
                    next_spot = col_index;
                }
                b'O' => {
                    next_spot -= 1;
                    text[row_index * width + col_index] = b'.';
                    text[row_index * width + next_spot] = b'O';
                }
                b'.' => {}
                _ => unreachable!(),
            }
        }
    }
}

fn slide_south(text: &mut [u8], width: usize, height: usize) {
    for col_index in 0..(width - 1) {
        let mut next_spot = height; // always +1 to be avoid underflow
        for row_index in (0..height).rev() {
            match text[row_index * width + col_index] {
                b'#' => {
                    next_spot = row_index;
                }
                b'O' => {
                    next_spot -= 1;
                    text[row_index * width + col_index] = b'.';
                    text[next_spot * width + col_index] = b'O';
                }
                b'.' => {}
                _ => unreachable!(),
            }
        }
    }
}

fn slide_west(text: &mut [u8], width: usize, height: usize) {
    for row_index in 0..height {
        let mut next_spot = 0u32;
        for col_index in 0..(width - 1) {
            match text[row_index * width + col_index] {
                b'#' => {
                    next_spot = col_index as u32 + 1;
                }
                b'O' => {
                    text[row_index * width + col_index] = b'.';
                    text[row_index * width + next_spot as usize] = b'O';
                    next_spot += 1;
                }
                b'.' => {}
                _ => unreachable!(),
            }
        }
    }
}
fn slide_north(text: &mut [u8], width: usize, height: usize) {
    for col_index in 0..(width - 1) {
        let mut next_spot = 0u32;
        for row_index in 0..height {
            match text[row_index * width + col_index] {
                b'#' => {
                    next_spot = row_index as u32 + 1;
                }
                b'O' => {
                    text[row_index * width + col_index] = b'.';
                    text[next_spot as usize * width + col_index] = b'O';
                    next_spot += 1;
                }
                b'.' => {}
                _ => unreachable!(),
            }
        }
    }
}

fn calc_score(text: &[u8], width: usize, height: usize) -> u32 {
    text.iter()
        .enumerate()
        .filter_map(|(index, &c)| {
            (c == b'O').then_some(height as u32 - index as u32 / width as u32)
        })
        .sum()
}
fn solve_file(text: String) -> u32 {
    let loops = 1000000000;
    let width = text.find('\n').unwrap() + 1;
    let height = (text.len() + 1) / width;
    let mut text = text.bytes().collect_vec();
    let mut hist: Vec<Vec<u8>> = Vec::new();
    for loop_index in 0..loops {
        if let Some(existing_pos) = hist.iter().position(|v| *v == text) {
            let cycle = &hist[existing_pos..];
            let remaining_loops = loops - loop_index;
            return calc_score(&cycle[remaining_loops % cycle.len()], width, height);
        }
        hist.push(text.clone());
        slide_north(&mut text, width, height);
        slide_west(&mut text, width, height);
        slide_south(&mut text, width, height);
        slide_east(&mut text, width, height);
    }
    calc_score(&text, width, height)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        assert_eq!(
            solve_file(read_to_string("inputs/day14_test.txt").unwrap()),
            64
        );
        assert_eq!(
            solve_file(read_to_string("inputs/day14.txt").unwrap()),
            104409
        );
    }
}
