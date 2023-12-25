use std::fs::read_to_string;

use itertools::Itertools;

pub fn solve_day() -> usize {
    solve_file(read_to_string("inputs/day25.txt").unwrap())
}
fn parse(text: String) -> Vec<Vec<u16>> {
    let mut text = text.as_bytes();
    let mut lookup_table = [u16::MAX; 26 * 26 * 26];
    let mut graph: Vec<Vec<u16>> = Vec::new();
    loop {
        if text.is_empty() {
            return graph;
        }
        let node_code = (text[0] - b'a') as u16 * 26 * 26
            + (text[1] - b'a') as u16 * 26
            + (text[2] - b'a') as u16;
        if lookup_table[node_code as usize] == u16::MAX {
            lookup_table[node_code as usize] = graph.len() as u16;
            graph.push(Vec::new());
        }
        let node_index = lookup_table[node_code as usize];
        // println!(
        //     "registered: {}{}{}:{}",
        //     text[0] as char, text[1] as char, text[2] as char, node_index
        // );
        text = &text[4..];
        while text[0] != b'\n' {
            let neigh_code = (text[1] - b'a') as u16 * 26 * 26
                + (text[2] - b'a') as u16 * 26
                + (text[3] - b'a') as u16;
            if lookup_table[neigh_code as usize] == u16::MAX {
                lookup_table[neigh_code as usize] = graph.len() as u16;
                graph.push(Vec::new());
            }
            let neigh_index = lookup_table[neigh_code as usize];
            graph[node_index as usize].push(neigh_index);
            graph[neigh_index as usize].push(node_index);
            text = &text[4..];
        }
        text = &text[1..];
    }
}
#[allow(dead_code)]
fn print_graph(graph: &[Vec<u16>]) {
    graph.iter().enumerate().for_each(|(i, node)| {
        println!("{}:{}", i, node.len());
        node.iter().for_each(|&neigh| print!("{},", neigh));
        println!();
    });
}
fn min_cut(graph: Vec<Vec<u16>>) -> usize {
    let mut visited = vec![false; graph.len()];
    visited[0] = true;
    let mut visited_count = 1;
    let mut distances = vec![0u16; graph.len()];
    for &neigh in &graph[0] {
        distances[neigh as usize] = 1;
    }
    let mut queue = graph[0].clone();
    let mut curr_cut = graph[0].len() as u16;
    while curr_cut != 3 {
        // println!("{:?}", queue);
        let most_tightly_connected_idx = queue
            .iter()
            // .inspect(|&&node_id| println!("{}:{}", node_id, distances[node_id as usize]))
            .position_max_by_key(|&&node_id| distances[node_id as usize])
            .unwrap();
        // move from queue to visited set
        let most_tightly_connected = queue.swap_remove(most_tightly_connected_idx) as usize;
        visited[most_tightly_connected] = true;
        visited_count += 1;
        // reduce cut by edges into the visited set
        curr_cut -= distances[most_tightly_connected];
        // increase cut by edges out of the visited set
        curr_cut += graph[most_tightly_connected].len() as u16 - distances[most_tightly_connected];
        // add neighbours to queue
        for &neigh in &graph[most_tightly_connected] {
            if !(queue.contains(&neigh) || visited[neigh as usize]) {
                queue.push(neigh);
                // there is a new edge into the visited set
            }
            distances[neigh as usize] += 1;
        }
    }
    // println!("curr cut:{}, size:{}", curr_cut, graph.len());
    visited_count * (graph.len() - visited_count)
}
fn solve_file(text: String) -> usize {
    let graph = parse(text);
    // print_graph(&graph);
    min_cut(graph)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        assert_eq!(
            solve_file(read_to_string("inputs/day25_test.txt").unwrap()),
            54
        );
        assert_eq!(
            solve_file(read_to_string("inputs/day25.txt").unwrap()),
            558376
        );
    }
}
