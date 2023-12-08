use std::collections::HashMap;
use std::{fs::File, io::BufRead, io::BufReader};

use ::num::integer::lcm;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

pub fn solve_day() -> usize {
    let file = File::open("inputs/day8.txt").unwrap();
    solve_file(file)
}
fn string_to_myhash(s: &str) -> usize {
    s.as_bytes()
        .iter()
        .fold(0usize, |acc, c| acc * 256 + *c as usize)
}
fn solve_file(file: File) -> usize {
    let mut lines = BufReader::new(file).lines();
    let mut current_nodes = Vec::<usize>::new();
    let route_str = lines.by_ref().next().unwrap().unwrap();
    // let mut graph: Vec<(usize, usize)> = vec![(0, 0); 256 * 256 * 256];
    let graph: HashMap<usize, (usize, usize)> = lines
        .skip(1)
        .map(|l| {
            let unwrapped_line = l.unwrap();
            let node_name = string_to_myhash(&unwrapped_line[..3]);
            let left_name = string_to_myhash(&unwrapped_line[7..10]);
            let right_name = string_to_myhash(&unwrapped_line[12..15]);
            // println!("{}", unwrapped_line);
            if node_name as u8 == b'A' {
                current_nodes.push(node_name);
                // println!("inserted {}", node_name)
            }
            (node_name, (left_name, right_name))
        })
        .collect();
    current_nodes
        .iter()
        .map(|n| solve_single(*n, &route_str, &graph))
        .reduce(lcm)
        .unwrap()
}

fn solve_single(mut node: usize, route_str: &str, graph: &HashMap<usize, (usize, usize)>) -> usize {
    route_str
        .chars()
        .cycle()
        .fold_while(0, |steps, direction| {
            let options = graph.get(&node).unwrap();
            node = match direction {
                'L' => options.0,
                'R' => options.1,
                _ => panic!("Impossible"),
            };
            if node as u8 == b'Z' {
                Done(steps + 1)
            } else {
                Continue(steps + 1)
            }
        })
        .into_inner()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        assert_eq!(solve_file(File::open("inputs/day8b_test.txt").unwrap()), 6);
        assert_eq!(
            solve_file(File::open("inputs/day8.txt").unwrap()),
            8245452805243
        );
    }
}
