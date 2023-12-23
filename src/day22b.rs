use std::{cmp::Ordering, fmt::Display, fs::read_to_string};
// use std::str;

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use smallvec::SmallVec;

#[derive(PartialEq, Eq, Debug)]
struct Brick {
    x1: u8,
    x2: u8,
    y1: u8,
    y2: u8,
    z1: u16,
    z2: u16,
    supporting: SmallVec<[u16; 8]>,
    supported_by: SmallVec<[u16; 8]>,
}
fn deep_supporting(bricks: &[Brick], start_id: u16) -> u16 {
    let mut queue = Vec::new();
    queue.push(start_id);
    let mut fallen_bricks = Vec::new();
    fallen_bricks.resize(bricks.len() - (start_id - 1) as usize, false);
    fallen_bricks[0] = true;
    let mut fallen_bricks_count = 0u16;
    while let Some(supported) = queue.pop() {
        let brick = &bricks[(supported - 1) as usize];
        brick.supporting.iter().for_each(|&brick_id| {
            let index = (brick_id - start_id) as usize;
            if !fallen_bricks[index]
                && bricks[brick_id as usize - 1]
                    .supported_by
                    .iter()
                    .all(|&support| {
                        support >= start_id && fallen_bricks[(support - start_id) as usize]
                    })
            {
                fallen_bricks[index] = true;
                fallen_bricks_count += 1;
                queue.push(brick_id);
            }
        });
    }
    fallen_bricks_count
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

pub fn solve_day() -> u32 {
    // solve_file(read_to_string("inputs/day22_test.txt").unwrap(), 3)
    solve_file(read_to_string("inputs/day22.txt").unwrap(), 10)
}
fn solve_file(text: String, size: usize) -> u32 {
    let mut text = text.as_bytes();
    let mut falling_bricks = Vec::<Brick>::new();
    let mut top_view = vec![PlacedBrick { id: 0, height: 0 }; size * size]; // height and id of top tile
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
            supporting: SmallVec::new(),
            supported_by: SmallVec::new(),
        });
    }
    falling_bricks.sort_unstable();
    // parsing done
    for brick_id in 0..falling_bricks.len() {
        // println!("dropping {:?}", falling_bricks[brick_id]);
        let mut max_height = 0u16;
        let brick = &mut falling_bricks[brick_id];
        let mut supported_by = SmallVec::<[u16; 8]>::new();
        for x in brick.x1..=brick.x2 {
            for y in brick.y1..=brick.y2 {
                let index = x as usize + y as usize * size;
                // println!("at x {} y {}: {}", x, y, top_view[index]);
                if top_view[index].id == 0 {
                    continue;
                }
                match top_view[index].height.cmp(&max_height) {
                    Ordering::Greater => {
                        max_height = top_view[index].height;
                        supported_by.clear();
                        if !supported_by.contains(&(top_view[index].id)) {
                            supported_by.push(top_view[index].id);
                        }
                    }
                    Ordering::Equal => {
                        if !supported_by.contains(&(top_view[index].id)) {
                            supported_by.push(top_view[index].id);
                        }
                    }
                    Ordering::Less => {}
                }
            }
        }
        let final_z2 = brick.z2 - (brick.z1 - max_height) + 1;
        // println!("final z2: {}", final_z2);
        // println!("supported_by: {:?}", &supported_by);
        // implicit brick drop
        for &support in supported_by.iter() {
            if !falling_bricks[support as usize - 1]
                .supporting
                .contains(&(brick_id as u16 + 1))
            {
                falling_bricks[support as usize - 1]
                    .supporting
                    .push(brick_id as u16 + 1);
            }
        }
        let brick = &mut falling_bricks[brick_id];
        brick.supported_by = supported_by;
        for x in brick.x1..=brick.x2 {
            for y in brick.y1..=brick.y2 {
                let index = x as usize + y as usize * size;
                top_view[index] = PlacedBrick {
                    id: brick_id as u16 + 1,
                    height: final_z2,
                };
            }
        }
        // print_grid(&top_view, size);
    }
    // dbg!(&falling_bricks);
    (1..=falling_bricks.len() as u16)
        .into_par_iter()
        .map(|brick_id| deep_supporting(&falling_bricks, brick_id) as u32)
        .sum()
}

#[allow(dead_code)]
fn print_grid(top_view: &[PlacedBrick], size: usize) {
    for y in 0..size {
        for x in 0..size {
            let index = x + size * y;
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
            solve_file(read_to_string("inputs/day22_test.txt").unwrap(), 3),
            7
        );
        assert_eq!(
            solve_file(read_to_string("inputs/day22.txt").unwrap(), 10),
            102770
        );
    }
}
