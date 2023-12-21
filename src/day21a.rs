use std::{collections::VecDeque, fs::read_to_string};

pub fn solve_day() -> u16 {
    // solve_file(read_to_string("inputs/day21_test.txt").unwrap(), 6)
    solve_file(read_to_string("inputs/day21.txt").unwrap(), 64)
}
fn solve_file(text: String, steps: u16) -> u16 {
    let text = text.into_bytes();
    let keep_remainder = steps % 2;
    let width = text.iter().position(|&c| c == b'\n').unwrap() + 1;
    let start = text.iter().position(|&c| c == b'S').unwrap();
    let mut visited = vec![false; text.len()];
    visited[start] = true;
    let mut queue = VecDeque::new();
    queue.push_back(start);
    let mut reachable_count = 0u16;
    for step in 0..steps {
        let iteration_size = queue.len();
        if step % 2 == keep_remainder {
            reachable_count += iteration_size as u16;
        }
        for _ in 0..iteration_size {
            let coord = queue.pop_front().unwrap();
            if text[coord + 1] == b'.' && !visited[coord + 1] {
                visited[coord + 1] = true;
                queue.push_back(coord + 1);
            }
            if coord > 0 && text[coord - 1] == b'.' && !visited[coord - 1] {
                visited[coord - 1] = true;
                queue.push_back(coord - 1);
            }
            if coord + width < text.len() && text[coord + width] == b'.' && !visited[coord + width]
            {
                visited[coord + width] = true;
                queue.push_back(coord + width);
            }
            if coord >= width && text[coord - width] == b'.' && !visited[coord - width] {
                visited[coord - width] = true;
                queue.push_back(coord - width);
            }
        }
    }
    // print_grid(&text, &visited);
    reachable_count + queue.len() as u16 // add final queue size
}

#[allow(dead_code)]
fn print_grid(text: &[u8], can_finish: &[bool]) {
    (0..can_finish.len()).for_each(|i| {
        if can_finish[i] {
            print!("@");
        } else {
            print!("{}", text[i] as char);
        }
    });
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        assert_eq!(
            solve_file(read_to_string("inputs/day21_test.txt").unwrap(), 6),
            16
        );
        assert_eq!(
            solve_file(read_to_string("inputs/day21.txt").unwrap(), 64),
            3733
        );
    }
}
