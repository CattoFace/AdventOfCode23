use std::cmp::Ordering;
use std::fmt::Display;
use std::fs::read_to_string;
// use std::str;

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
struct Brick {
    x1: u8,
    x2: u8,
    y1: u8,
    y2: u8,
    z1: u16,
    z2: u16,
    supporting: bool,
}
#[derive(Clone)]
struct PlacedBrick {
    id: u16,
    height: u16,
}
impl Display for PlacedBrick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:^3},{:^2})", self.id, self.height)
    }
}
#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.z1.partial_cmp(&other.z1)
    }
}
impl Ord for Brick {
    fn cmp(&self, other: &Brick) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn solve_day() -> u16 {
    solve_file(read_to_string("inputs/day22.txt").unwrap())
}
const SIZE: usize = 10;
fn solve_file(text: String) -> u16 {
    let mut text = text.as_bytes();
    let mut falling_bricks = Vec::<Brick>::new();
    let mut top_view = vec![PlacedBrick { id: 0, height: 0 }; SIZE * SIZE]; // height and id of top tile
    loop {
        if text.is_empty() {
            break;
        }
        let x1 = text[0] - b'0';
        let y1 = text[2] - b'0';
        text = &text[4..];
        let mut i = 0usize;
        let z1 = text
            .iter()
            .fold_while(0u16, |acc, &c| {
                i += 1;
                if c == b'~' {
                    Done(acc)
                } else {
                    Continue(acc * 10 + (c - b'0') as u16)
                }
            })
            .into_inner();
        text = &text[i..];
        let x2 = text[0] - b'0';
        let y2 = text[2] - b'0';
        text = &text[4..];
        let mut i = 0usize;
        let z2 = text
            .iter()
            .fold_while(0u16, |acc, &c| {
                i += 1;
                if c == b'\n' {
                    Done(acc)
                } else {
                    Continue(acc * 10 + (c - b'0') as u16)
                }
            })
            .into_inner();
        text = &text[i..];
        falling_bricks.push(Brick {
            x1,
            x2,
            y1,
            y2,
            z1,
            z2,
            supporting: false,
        });
    }
    falling_bricks.sort_unstable();
    for brick_id in 0..falling_bricks.len() {
        let mut max_height = 0u16;
        let brick = &falling_bricks[brick_id];
        // println!("dropping brick {}:{:?}", brick_id + 1, brick);
        let mut loadbearing_brick = None;
        for x in brick.x1..=brick.x2 {
            for y in brick.y1..=brick.y2 {
                let index = x as usize + y as usize * SIZE;
                // println!("at x {} y {}: {}", x, y, top_view[index]);
                match top_view[index].height.cmp(&max_height) {
                    Ordering::Greater => {
                        loadbearing_brick = Some(top_view[index].id);
                        max_height = top_view[index].height;
                    }
                    Ordering::Equal => {
                        if loadbearing_brick.is_some_and(|id| id != top_view[index].id) {
                            loadbearing_brick = None
                        }
                    } // there is more than 1 supporting
                    // brick, so neither are load bearing
                    Ordering::Less => {}
                }
            }
        }
        let final_z2 = brick.z2 - (brick.z1 - max_height) + 1;
        // println!("final z2: {}", final_z2);
        // implicit brick drop
        if let Some(loadbearing_id) = loadbearing_brick {
            // println!("loadbearing: {}", loadbearing_id);
            falling_bricks[(loadbearing_id - 1) as usize].supporting = true
        }
        let brick = &falling_bricks[brick_id];
        for x in brick.x1..=brick.x2 {
            for y in brick.y1..=brick.y2 {
                let index = x as usize + y as usize * SIZE;
                top_view[index] = PlacedBrick {
                    id: brick_id as u16 + 1,
                    height: final_z2,
                };
            }
        }
        // print_grid(&top_view);
    }
    // dbg!(&falling_bricks);
    falling_bricks
        .iter()
        .filter(|&&brick| !brick.supporting)
        .count() as u16
}

#[allow(dead_code)]
fn print_grid(top_view: &[PlacedBrick]) {
    for y in 0..SIZE {
        for x in 0..SIZE {
            let index = x + SIZE * y;
            print!("{}", top_view[index])
        }
        println!();
    }
    println!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        assert_eq!(
            solve_file(read_to_string("inputs/day22_test.txt").unwrap()),
            5
        );
        assert_eq!(solve_file(read_to_string("inputs/day22.txt").unwrap()), 509);
    }
}
