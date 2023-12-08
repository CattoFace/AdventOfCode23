use std::{collections::HashMap, fs::File, io::BufRead, io::BufReader};

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

pub fn solve_day() -> u32 {
    let file = File::open("inputs/day8.txt").unwrap();
    solve_file(file)
}
fn string_to_myhash(s: &str) -> u32 {
    s.as_bytes()
        .iter()
        .fold(0u32, |acc, c| acc * 256 + *c as u32)
}

fn solve_file(file: File) -> u32 {
    let mut lines = BufReader::new(file).lines();
    let route_str = lines.by_ref().next().unwrap().unwrap();
    let graph: HashMap<u32, (u32, u32)> = lines
        .skip(1)
        .map(|l| {
            let unwrapped_line = l.unwrap();
            let node_name = string_to_myhash(&unwrapped_line[..3]);
            let left_name = string_to_myhash(&unwrapped_line[7..10]);
            let right_name = string_to_myhash(&unwrapped_line[12..15]);
            (node_name, (left_name, right_name))
        })
        .collect();
    let target = string_to_myhash("ZZZ");
    route_str
        .chars()
        .cycle()
        .fold_while(
            (string_to_myhash("AAA"), 0u32),
            |(current_node, steps), direction| {
                if current_node == target {
                    return Done((current_node, steps));
                }

                let options = graph.get(&current_node).unwrap();
                Continue((
                    match direction {
                        'L' => options.0,
                        'R' => options.1,
                        _ => panic!("Impossible"),
                    },
                    steps + 1,
                ))
            },
        )
        .into_inner()
        .1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        assert_eq!(solve_file(File::open("inputs/day8_test.txt").unwrap()), 2);
        assert_eq!(solve_file(File::open("inputs/day8.txt").unwrap()), 12599);
    }
}
