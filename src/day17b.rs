use bitvec::prelude::*;
use colored::Colorize;
use std::{collections::BinaryHeap, fs::read_to_string};
pub fn solve_day() -> u16 {
    solve_file(read_to_string("inputs/day17.txt").unwrap())
}
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Direction {
    Horizontal,
    Vertical,
}

#[derive(PartialEq, Eq, Debug)]
struct DistrictMove {
    x: u8,
    y: u8,
    cost: u16,
    direction: Direction,
}

impl Ord for DistrictMove {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for DistrictMove {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[allow(dead_code)]
fn print_grid(text: &[u8], index: usize) {
    println!();
    text.iter().enumerate().for_each(|(i, &c)| {
        if i == index {
            print!("{}", "O".red());
        } else {
            print!("{}", c as char);
        }
    });
    println!();
}
fn solve_file(text: String) -> u16 {
    use Direction::*;
    let text = text.as_bytes();
    let width = text.iter().position(|&c| c == b'\n').unwrap() as u8;
    let height = (text.len() as u16 / (width + 1) as u16) as u8;
    let mut visited = bitvec![0;text.len()*2]; // 2 axis
    let mut queue = BinaryHeap::<DistrictMove>::new();
    queue.push(DistrictMove {
        x: 0,
        y: 0,
        cost: 0,
        direction: Vertical,
    });
    queue.push(DistrictMove {
        x: 0,
        y: 0,
        cost: 0,
        direction: Horizontal,
    });
    loop {
        let curr_district = queue.pop().unwrap();
        let index = curr_district.x as usize + curr_district.y as usize * (width + 1) as usize;
        let visited_index = index + curr_district.direction as usize * text.len();
        // print_grid(text, index);
        // println!("{:?}", queue);
        // let mut s = String::new();
        // std::io::stdin().read_line(&mut s);
        if visited[visited_index] {
            continue;
        }
        visited.set(visited_index, true);
        if curr_district.x == width - 1 && curr_district.y == height - 1 {
            return curr_district.cost;
        }
        if !matches!(curr_district.direction, Horizontal) {
            let mut cost = curr_district.cost;
            for jump in 1u8..=10u8 {
                if curr_district.y >= jump {
                    cost += (text[index - jump as usize * (width as usize + 1)] - b'0') as u16;
                    if jump >= 4 {
                        queue.push(DistrictMove {
                            x: curr_district.x,
                            y: curr_district.y - jump,
                            cost,
                            direction: Horizontal,
                        });
                    }
                } else {
                    break;
                }
            }
            let mut cost = curr_district.cost;
            for jump in 1u8..=10u8 {
                if curr_district.y + jump < height {
                    cost += (text[index + jump as usize * (width as usize + 1)] - b'0') as u16;
                    if jump >= 4 {
                        queue.push(DistrictMove {
                            x: curr_district.x,
                            y: curr_district.y + jump,
                            cost,
                            direction: Horizontal,
                        });
                    }
                } else {
                    break;
                }
            }
        }
        if !matches!(curr_district.direction, Vertical) {
            let mut cost = curr_district.cost;
            for jump in 1u8..=10u8 {
                if curr_district.x >= jump {
                    cost += (text[index - jump as usize] - b'0') as u16;
                    if jump >= 4 {
                        queue.push(DistrictMove {
                            x: curr_district.x - jump,
                            y: curr_district.y,
                            cost,
                            direction: Vertical,
                        });
                    }
                } else {
                    break;
                }
            }
            let mut cost = curr_district.cost;
            for jump in 1u8..=10u8 {
                if curr_district.x + jump < width {
                    cost += (text[index + jump as usize] - b'0') as u16;
                    if jump >= 4 {
                        queue.push(DistrictMove {
                            x: curr_district.x + jump,
                            y: curr_district.y,
                            cost,
                            direction: Vertical,
                        });
                    }
                } else {
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        assert_eq!(
            solve_file(read_to_string("inputs/day17_test.txt").unwrap()),
            102
        );
        assert_eq!(solve_file(read_to_string("inputs/day17.txt").unwrap()), 936);
    }
}
