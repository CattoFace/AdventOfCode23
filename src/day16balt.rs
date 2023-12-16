use std::fs::read_to_string;

use rayon::prelude::{IntoParallelIterator, ParallelIterator};

pub fn solve_day() -> u32 {
    solve_file(read_to_string("inputs/day16.txt").unwrap())
}
#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn ray_init(
    text: &[u8],
    width: usize,
    height: usize,
    x: usize,
    y: usize,
    direction: Direction,
) -> u32 {
    let mut registry = vec![false; text.len()];
    ray(text, &mut registry, width, height, x, y, direction)
}
fn ray(
    text: &[u8],
    registry: &mut [bool],
    width: usize,
    height: usize,
    mut x: usize,
    mut y: usize,
    mut direction: Direction,
) -> u32 {
    use Direction::*;
    let mut ans = 0u32;
    let mut index = width * y + x;
    // 8th bit marks energized
    while !(registry[index] && (text[index] == b'-' || text[index] == b'|')) {
        ans += !registry[index] as u32;
        registry[index] = true;
        match (text[index], &direction) {
            (b'.' | b'|', Up) => {
                if y == 0 {
                    break;
                };
                y -= 1;
            }
            (b'.' | b'|', Down) => {
                if y == height - 1 {
                    break;
                }
                y += 1;
            }
            (b'.' | b'-', Left) => {
                if x == 0 {
                    break;
                }
                x -= 1;
            }
            (b'.' | b'-', Right) => {
                if x == width - 2 {
                    break;
                }
                x += 1;
            }
            (b'\\', Up) | (b'/', Down) => {
                if x == 0 {
                    break;
                }
                x -= 1;
                direction = Left;
            }
            (b'\\', Down) | (b'/', Up) => {
                if x == width - 2 {
                    break;
                }
                x += 1;
                direction = Right
            }
            (b'\\', Left) | (b'/', Right) => {
                if y == 0 {
                    break;
                }
                y -= 1;
                direction = Up
            }
            (b'\\', Right) | (b'/', Left) => {
                if y == height - 1 {
                    break;
                }
                y += 1;
                direction = Down
            }
            (b'-', Up | Down) => {
                if x < width - 2 {
                    ans += ray(text, registry, width, height, x + 1, y, Right);
                }
                if x == 0 {
                    break;
                }
                x -= 1;
                direction = Left;
            }
            (b'|', Left | Right) => {
                if y < height - 1 {
                    ans += ray(text, registry, width, height, x, y + 1, Down);
                }
                if y == 0 {
                    break;
                }
                y -= 1;
                direction = Up;
            }
            _ => unreachable!(
                "Reached with {},{},{},{},{:?}",
                x, y, text[index] as char, registry[index], direction
            ),
        }
        index = width * y + x;
        // print_grid(text, index);
    }
    ans
}

#[allow(dead_code)]
fn print_grid(text: &[u8], index: usize) {
    println!();
    text.iter().enumerate().for_each(|(i, &c)| {
        if i == index {
            print!("O");
        } else if c == 0 {
            print!("{}", c as char);
        } else {
            print!("#");
        }
    })
}
fn solve_file(text: String) -> u32 {
    let mut binding = text.into_bytes();
    let text: &mut [u8] = binding.as_mut();
    let width = text.iter().position(|&c| c == b'\n').unwrap() + 1;
    let height = text.len() / width;
    let col_max = (0..(width - 1))
        .into_par_iter()
        .map(|col_idx| {
            ray_init(text, width, height, col_idx, 0, Direction::Down).max(ray_init(
                text,
                width,
                height,
                col_idx,
                height - 1,
                Direction::Up,
            ))
        })
        .max()
        .unwrap();
    let row_max = (0..height)
        .into_par_iter()
        .map(|row_idx| {
            ray_init(text, width, height, 0, row_idx, Direction::Right).max(ray_init(
                text,
                width,
                height,
                width - 2,
                row_idx,
                Direction::Left,
            ))
        })
        .max()
        .unwrap();
    row_max.max(col_max)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        assert_eq!(
            solve_file(read_to_string("inputs/day16_test.txt").unwrap()),
            51
        );
        assert_eq!(
            solve_file(read_to_string("inputs/day16.txt").unwrap()),
            7987
        );
    }
}
