use std::{array::from_fn, fs::read_to_string, str};

pub fn solve_day() -> u32 {
    solve_file(read_to_string("inputs/day15.txt").unwrap())
}

fn hash(text: &[u8]) -> u8 {
    text.iter()
        .fold(0u8, |acc, &c| acc.wrapping_add(c).wrapping_mul(17))
}
fn solve_file(text: String) -> u32 {
    let mut boxes: [Vec<(&[u8], &[u8])>; 256] = from_fn(|_| Vec::new());
    let instructions = text.as_bytes()[..(text.len() - 1)].split(|&c| c == b',');
    instructions.for_each(|instruction| {
        let split_index = instruction
            .iter()
            .position(|&c| c == b'=' || c == b'-')
            .unwrap();
        let (label, mut length) = instruction.split_at(split_index);
        let index = hash(label);
        length = &length[1..]; // ending line feed
        let existing = boxes[index as usize].iter().position(|(l, _)| l == &label);
        match instruction[split_index] {
            b'=' => {
                if let Some(to_add) = existing {
                    boxes[index as usize][to_add] = (label, length);
                } else {
                    boxes[index as usize].push((label, length));
                }
            }
            b'-' => {
                if let Some(to_remove) = existing {
                    boxes[index as usize].remove(to_remove);
                }
            }
            _ => unreachable!(),
        }
    });
    boxes
        .iter()
        .enumerate()
        .flat_map(|(box_idx, lenses)| {
            lenses.iter().enumerate().map(move |(idx, lens)| {
                (box_idx as u32 + 1)
                    * (idx as u32 + 1)
                    * str::from_utf8(lens.1).unwrap().parse::<u32>().unwrap()
            })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        assert_eq!(
            solve_file(read_to_string("inputs/day15_test.txt").unwrap()),
            145
        );
        assert_eq!(
            solve_file(read_to_string("inputs/day15.txt").unwrap()),
            516469
        );
    }
}
