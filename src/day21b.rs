use std::{collections::VecDeque, fs::read_to_string};

pub fn solve_day() -> usize {
    // solve_file(read_to_string("inputs/day21b_test.txt").unwrap(), 40)
    solve_file(read_to_string("inputs/day21.txt").unwrap(), 26501365)
}

fn explore_grid(
    grid: &[u8],
    start: usize,
    steps: usize,
    width: usize,
    mut keep_start: bool,
) -> usize {
    let mut visited = vec![false; grid.len()];
    let mut can_finish = vec![false; grid.len()];
    visited[start] = true;
    can_finish[start] = keep_start; // can only keep start if steps are even
    let mut count = keep_start as usize;
    let mut queue = VecDeque::new();
    queue.push_back(start);
    for _ in 0..steps {
        if queue.is_empty() {
            break;
        }
        let iteration_size = queue.len();
        keep_start = !keep_start;
        for _ in 0..iteration_size {
            let coord = queue.pop_front().unwrap();
            if grid[coord + 1] == b'.' && !visited[coord + 1] {
                if keep_start {
                    count += 1;
                    can_finish[coord + 1] = true;
                }
                visited[coord + 1] = true;
                queue.push_back(coord + 1);
            }
            if coord > 0 && grid[coord - 1] == b'.' && !visited[coord - 1] {
                if keep_start {
                    count += 1;
                    can_finish[coord - 1] = true;
                }
                visited[coord - 1] = true;
                queue.push_back(coord - 1);
            }
            if coord + width < grid.len() && grid[coord + width] == b'.' && !visited[coord + width]
            {
                if keep_start {
                    count += 1;
                    can_finish[coord + width] = true;
                }
                visited[coord + width] = true;
                queue.push_back(coord + width);
            }
            if coord >= width && grid[coord - width] == b'.' && !visited[coord - width] {
                if keep_start {
                    count += 1;
                    can_finish[coord - width] = true;
                }
                visited[coord - width] = true;
                queue.push_back(coord - width);
            }
        }
    }
    // print_grid(grid, &can_finish);
    count
}
fn solve_file(text: String, steps: usize) -> usize {
    // println!("{}", text);
    let mut text = text.into_bytes();
    let mut can_finish = steps % 2 == 0;
    let width = text.iter().position(|&c| c == b'\n').unwrap() + 1;
    let height = text.len() / width;
    let start = text.iter().position(|&c| c == b'S').unwrap();
    text[start] = b'.';
    let mut visited = vec![false; text.len()];
    let mut even_map = vec![false; text.len()];
    visited[start] = true;
    even_map[start] = can_finish; // can only keep start if steps are even
    let mut even_grid = can_finish as usize;
    let mut queue = VecDeque::new();
    queue.push_back(start);
    for _ in 0.. {
        if queue.is_empty() {
            break;
        }
        let iteration_size = queue.len();
        can_finish = !can_finish;
        for _ in 0..iteration_size {
            let coord = queue.pop_front().unwrap();
            if text[coord + 1] == b'.' && !visited[coord + 1] {
                if can_finish {
                    even_grid += 1;
                    even_map[coord + 1] = true;
                }
                visited[coord + 1] = true;
                queue.push_back(coord + 1);
            }
            if coord > 0 && text[coord - 1] == b'.' && !visited[coord - 1] {
                if can_finish {
                    even_grid += 1;
                    even_map[coord - 1] = true;
                }
                visited[coord - 1] = true;
                queue.push_back(coord - 1);
            }
            if coord + width < text.len() && text[coord + width] == b'.' && !visited[coord + width]
            {
                if can_finish {
                    even_grid += 1;
                    even_map[coord + width] = true;
                }
                visited[coord + width] = true;
                queue.push_back(coord + width);
            }
            if coord >= width && text[coord - width] == b'.' && !visited[coord - width] {
                if can_finish {
                    even_grid += 1;
                    even_map[coord - width] = true;
                }
                visited[coord - width] = true;
                queue.push_back(coord - width);
            }
        }
    }
    // print_grid(&text, &even_map);
    let odd_grid = explore_grid(&text, start, width * width, width, steps % 2 != 0);
    let to_first_edge = width / 2;
    let left_after_edge = steps - to_first_edge;
    let left_after_corner = steps - 2 * to_first_edge;
    let filled_radius = left_after_corner / height;
    let left_to_outer_straight = left_after_edge % height;
    let left_to_outer_corner = left_after_corner % height;
    let odd_filled_grids_count = ((filled_radius + 1) & !1).pow(2);
    let even_filled_grids_count = (filled_radius | 1).pow(2);
    let edge_keep_start = (filled_radius + 1) % 2 == 0;
    // dbg!(
    //     &steps,
    //     &width,
    //     &height,
    //     &even_grid,
    //     &odd_grid,
    //     &to_first_edge,
    //     &left_after_edge,
    //     &left_after_corner,
    //     &filled_radius,
    //     &left_to_outer_straight,
    //     &left_to_outer_corner,
    //     &odd_filled_grids_count,
    //     &even_filled_grids_count,
    // );
    let left_grid = if left_to_outer_straight < (height / 2) {
        explore_grid(
            &text,
            width * ((height - 1) / 2),
            left_to_outer_straight,
            width,
            !edge_keep_start,
        ) + explore_grid(
            &text,
            width * ((height - 1) / 2),
            height + left_to_outer_straight,
            width,
            edge_keep_start,
        )
    } else {
        explore_grid(
            &text,
            width * ((height - 1) / 2),
            left_to_outer_straight,
            width,
            edge_keep_start,
        )
    };
    let right_grid = if left_to_outer_straight < height / 2 {
        explore_grid(
            &text,
            width * (height + 1) / 2 - 2,
            left_to_outer_straight,
            width,
            !edge_keep_start,
        ) + explore_grid(
            &text,
            width * ((height + 1) / 2) - 2,
            height + left_to_outer_straight,
            width,
            edge_keep_start,
        )
    } else {
        explore_grid(
            &text,
            width * (height + 1) / 2 - 2,
            left_to_outer_straight,
            width,
            edge_keep_start,
        )
    };
    let top_grid = if left_to_outer_straight < height / 2 {
        explore_grid(
            &text,
            width / 2 - 1,
            left_to_outer_straight,
            width,
            !edge_keep_start,
        ) + explore_grid(
            &text,
            width / 2 - 1,
            height + left_to_outer_straight,
            width,
            edge_keep_start,
        )
    } else {
        explore_grid(
            &text,
            width / 2 - 1,
            left_to_outer_straight,
            width,
            edge_keep_start,
        )
    };
    let bottom_grid = if left_to_outer_straight < height / 2 {
        explore_grid(
            &text,
            width * (height - 1) + width / 2 - 1,
            left_to_outer_straight,
            width,
            !edge_keep_start,
        ) + explore_grid(
            &text,
            width * (height - 1) + width / 2 - 1,
            height + left_to_outer_straight,
            width,
            edge_keep_start,
        )
    } else {
        explore_grid(
            &text,
            width * (height - 1) + width / 2 - 1,
            left_to_outer_straight,
            width,
            edge_keep_start,
        )
    };
    let bottom_left_grid = explore_grid(
        &text,
        width * (height - 1),
        left_to_outer_corner,
        width,
        edge_keep_start,
    );
    let bottom_left_outer_grid = explore_grid(
        &text,
        width * (height - 1),
        height + left_to_outer_corner,
        width,
        !edge_keep_start,
    );
    let bottom_right_grid = explore_grid(
        &text,
        width * (height - 1) + height - 1,
        left_to_outer_corner,
        width,
        edge_keep_start,
    );
    let bottom_right_outer_grid = explore_grid(
        &text,
        width * (height - 1) + height - 1,
        height + left_to_outer_corner,
        width,
        !edge_keep_start,
    );

    let top_left_grid = explore_grid(&text, 0, left_to_outer_corner, width, edge_keep_start);
    let top_left_outer_grid = explore_grid(
        &text,
        0,
        height + left_to_outer_corner,
        width,
        !edge_keep_start,
    );
    let top_right_grid = explore_grid(
        &text,
        height - 1,
        left_to_outer_corner,
        width,
        edge_keep_start,
    );
    let top_right_outer_grid = explore_grid(
        &text,
        height - 1,
        height + left_to_outer_corner,
        width,
        !edge_keep_start,
    );
    // dbg!(
    //     &left_grid,
    //     &right_grid,
    //     &top_grid,
    //     &bottom_grid,
    //     &bottom_left_grid,
    //     &bottom_left_outer_grid,
    //     &bottom_right_grid,
    //     &bottom_right_outer_grid,
    //     &top_left_grid,
    //     &top_left_outer_grid,
    //     &top_right_grid,
    //     &top_right_outer_grid
    // );
    odd_grid * odd_filled_grids_count
        + even_grid * even_filled_grids_count
        + left_grid
        + right_grid
        + top_grid
        + bottom_grid
        + (bottom_left_grid + bottom_right_grid + top_left_grid + top_right_grid)
            * (filled_radius + edge_keep_start as usize)
        + (bottom_left_outer_grid
            + bottom_right_outer_grid
            + top_right_outer_grid
            + top_left_outer_grid)
            * (filled_radius + !edge_keep_start as usize)
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
            solve_file(read_to_string("inputs/day21.txt").unwrap(), 26501365),
            617729401414635
        );
    }
}
