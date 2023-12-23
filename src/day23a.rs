use std::{collections::HashMap, fs::read_to_string};
#[derive(Copy, Clone)]
struct Edge {
    dest: u16,
    cost: u16,
}
fn explore_edge(text: &[u8], cache: &mut HashMap<u16, Edge>, start_coord: u16, width: u16) -> Edge {
    if let Some(exists) = cache.get(&start_coord) {
        return *exists;
    }
    let mut coord = start_coord;
    let mut cost = 0u16;
    let mut last_coord = coord;
    loop {
        // println!("{}:{},{}", coord, coord / width, coord % width);
        if text[coord as usize] != b'.' || coord == text.len() as u16 - 3 {
            let edge = Edge {
                dest: 2 * coord - last_coord, // to go in the same direction again
                cost: cost + 1,
            };
            cache.insert(start_coord, edge);
            return edge;
        }
        cost += 1;
        if coord - 1 != last_coord && text[coord as usize - 1] != b'#' {
            last_coord = coord;
            coord -= 1;
            continue;
        }
        if coord + 1 != last_coord && text[coord as usize + 1] != b'#' {
            last_coord = coord;
            coord += 1;
            continue;
        }
        if coord - width != last_coord && text[(coord - width) as usize] != b'#' {
            last_coord = coord;
            coord -= width;
            continue;
        }
        if coord + width != last_coord && text[(coord + width) as usize] != b'#' {
            last_coord = coord;
            coord += width;
            continue;
        }
    }
}

pub fn solve_day() -> u16 {
    solve_file(read_to_string("inputs/day23.txt").unwrap())
}
fn solve_file(text: String) -> u16 {
    let mut cache = HashMap::<u16, Edge>::new();
    let mut text = text.into_bytes();
    let width = (text.iter().position(|&c| c == b'\n').unwrap() + 1) as u16;
    let mut queue = Vec::new();
    let mut longest = 0u16;
    text[1] = b'#';
    let edge = explore_edge(&text, &mut cache, width + 1, width);
    queue.push((edge.dest, edge.cost + 1));
    while let Some((coord, curr_cost)) = queue.pop() {
        // println!("{},{}:{}", coord / width, coord % width, curr_cost);
        if coord == text.len() as u16 + width - 3 {
            longest = longest.max(curr_cost - 1);
            continue;
        }
        if text[coord as usize + 1] == b'>' {
            text[coord as usize + 1] = b'#';
            let edge = explore_edge(&text, &mut cache, coord + 2, width);
            text[coord as usize + 1] = b'>';
            queue.push((edge.dest, curr_cost + edge.cost + 2));
        }
        if text[(coord + width) as usize] == b'v' {
            text[(coord + width) as usize] = b'#';
            let edge = explore_edge(&text, &mut cache, coord + 2 * width, width);
            text[(coord + width) as usize] = b'v';
            queue.push((edge.dest, curr_cost + edge.cost + 2));
        }
        if text[(coord - width) as usize] == b'^' {
            text[(coord - width) as usize] = b'#';
            let edge = explore_edge(&text, &mut cache, coord - 2 * width, width);
            text[(coord - width) as usize] = b'^';
            queue.push((edge.dest, curr_cost + edge.cost + 2));
        }
    }
    longest
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        assert_eq!(
            solve_file(read_to_string("inputs/day23_test.txt").unwrap()),
            94
        );
        assert_eq!(
            solve_file(read_to_string("inputs/day23.txt").unwrap()),
            2250
        );
    }
}
