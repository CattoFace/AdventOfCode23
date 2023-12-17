use bitvec::prelude::*;
use colored::Colorize;
use std::{collections::BinaryHeap, fs::read_to_string};
pub fn solve_day() -> u32 {
    solve_file(read_to_string("inputs/day17.txt").unwrap())
}
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq, Debug)]
struct DistrictMove {
    x: u8,
    y: u8,
    cost: u32,
    direction: Direction,
    move_length: u8,
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

fn next_district(
    text: &[u8],
    curr_district: &DistrictMove,
    direction: Direction,
    width: u8,
) -> DistrictMove {
    let mut x = curr_district.x;
    let mut y = curr_district.y;
    match direction {
        Direction::Up => y -= 1,
        Direction::Down => y += 1,
        Direction::Left => x -= 1,
        Direction::Right => x += 1,
    }
    // println!(
    //     "{},{},{}",
    //     text[(x + y * (width + 1)) as usize] as char,
    //     x,
    //     y
    // );
    DistrictMove {
        x,
        y,
        cost: curr_district.cost
            + (text[(x as u32 + y as u32 * (width + 1) as u32) as usize] - b'0') as u32,
        direction,
        move_length: if curr_district.direction == direction {
            curr_district.move_length + 1
        } else {
            1
        },
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
fn solve_file(text: String) -> u32 {
    use Direction::*;
    let text = text.as_bytes();
    let width = text.iter().position(|&c| c == b'\n').unwrap() as u8;
    let height = (text.len() as u32 / (width + 1) as u32) as u8;
    let mut visited = bitvec![0;text.len() * 4 * 3]; // 4 directions, 4 move_lengths
    let mut queue = BinaryHeap::<DistrictMove>::new();
    queue.push(DistrictMove {
        x: 0,
        y: 1,
        cost: (text[(width + 1) as usize] - b'0') as u32,
        direction: Down,
        move_length: 1,
    });
    queue.push(DistrictMove {
        x: 1,
        y: 0,
        cost: (text[1] - b'0') as u32,
        direction: Right,
        move_length: 1,
    });
    loop {
        let curr_district = queue.pop().unwrap();
        let index = (curr_district.x as u32
            + curr_district.y as u32 * (width + 1) as u32
            + curr_district.direction as u32 * text.len() as u32
            + (curr_district.move_length-1) as u32 * text.len() as u32 * 4)
            as usize;
        if visited[index] {
            // println!("thrown {},{}", curr_district.x, curr_district.y);
            // let mut s = String::new();
            // std::io::stdin().read_line(&mut s);
            continue;
        }
        visited.set(index, true);
        if curr_district.x == width - 1 && curr_district.y == height - 1 {
            return curr_district.cost;
        }
        if curr_district.y != 0
            && !matches!(curr_district.direction, Down)
            && (!matches!(curr_district.direction, Up) || curr_district.move_length < 3)
        {
            queue.push(next_district(text, &curr_district, Up, width));
        }
        if curr_district.y != height - 1
            && !matches!(curr_district.direction, Up)
            && (!matches!(curr_district.direction, Down) || curr_district.move_length < 3)
        {
            queue.push(next_district(text, &curr_district, Down, width));
        }
        if curr_district.x != 0
            && !matches!(curr_district.direction, Right)
            && (!matches!(curr_district.direction, Left) || curr_district.move_length < 3)
        {
            queue.push(next_district(text, &curr_district, Left, width));
        }
        if curr_district.x != width - 1
            && !matches!(curr_district.direction, Left)
            && (!matches!(curr_district.direction, Right) || curr_district.move_length < 3)
        {
            queue.push(next_district(text, &curr_district, Right, width));
        }
        // print_grid(
        //     text,
        //     (curr_district.x + curr_district.y * (width + 1)) as usize,
        // );
        // println!("{:?}", queue);
        // let mut s = String::new();
        // std::io::stdin().read_line(&mut s);
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
