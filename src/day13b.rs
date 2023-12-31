use std::fs::read_to_string;

pub fn solve_day() -> u32 {
    solve_file(read_to_string("inputs/day13.txt").unwrap())
}

fn split_equal(line: &[u8], pos: usize) -> (&[u8], &[u8]) {
    let size = pos.min(line.len() - pos);
    (&line[(pos - size)..pos], &line[pos..(pos + size)])
}
fn process_pattern(pattern: &str) -> u32 {
    if pattern.is_empty() {
        return 0;
    }
    let width = pattern.find('\n').unwrap() + 1;
    let pattern = pattern.as_bytes();
    let height = (pattern.len() + 1) / width;
    // check every possible column mirror
    if let Some(col_mirror) = (1..(width - 1)).position(|mirror_index| {
        // check every line matches the column mirror
        (0..height)
            .flat_map(|line_index| {
                let line = &pattern[(width * line_index)..(width * (line_index + 1) - 1)];
                let (left, right) = split_equal(line, mirror_index);
                let ans = left.iter().zip(right.iter().rev()).filter(|(a, b)| a != b);
                ans
            })
            .take(2)
            .count()
            == 1usize
    }) {
        return col_mirror as u32 + 1;
    }
    // check every possible row mirror
    ((1..height)
        .position(|index| {
            let size = index.min(height - index);
            ((index - size)..index)
                .zip((index..(index + size)).rev())
                .flat_map(|(l1, l2)| {
                    let top = &pattern[(width * l1)..(width * (l1 + 1) - 1)];
                    let bottom = &pattern[(width * l2)..(width * (l2 + 1) - 1)];
                    top.iter().zip(bottom.iter()).filter(|(a, b)| a != b)
                })
                .take(2)
                .count()
                == 1usize
        })
        .unwrap() as u32
        + 1)
        * 100
}

fn solve_file(text: String) -> u32 {
    text.split("\n\n").map(process_pattern).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        assert_eq!(
            solve_file(read_to_string("inputs/day13_test.txt").unwrap()),
            400
        );
        assert_eq!(
            solve_file(read_to_string("inputs/day13.txt").unwrap()),
            30449
        );
    }
}
