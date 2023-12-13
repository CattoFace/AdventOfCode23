use std::fs::read_to_string;

pub fn solve_day() -> u32 {
    solve_file(read_to_string("inputs/day13.txt").unwrap())
}

fn process_pattern(pattern: &str) -> u32 {
    if pattern.is_empty() {
        return 0;
    }
    let width = pattern.find('\n').unwrap();
    let height = pattern.len() / (width + 1) + 1;
    let mut col_options = vec![true; width - 1];
    pattern
        .lines()
        .for_each(|line| process_line_col(line, &mut col_options));
    if let Some(col_line) = col_options.iter().position(|o| *o) {
        return col_line as u32 + 1;
    }
    let mut row_options = vec![true; height - 1];
    for c in 0..width {
        process_line_row(pattern, c, width, &mut row_options);
    }
    (row_options.iter().position(|o| *o).unwrap() as u32 + 1) * 100
}

fn process_line_row(pattern: &str, column: usize, width: usize, col_options: &mut [bool]) {
    for c in 1..=col_options.len() {
        let remaining = col_options.len() - c + 1;
        let size = remaining.min(c);
        col_options[c - 1] = col_options[c - 1] && {
            let mut ans = true;
            for i in 0..size {
                if pattern.as_bytes()[column + (width + 1) * (c - size + i)]
                    != pattern.as_bytes()[column + (width + 1) * (c + size - i - 1)]
                {
                    ans = false;
                    break;
                }
            }
            ans
        };
    }
}
fn process_line_col(line: &str, col_options: &mut [bool]) {
    for c in 1..=col_options.len() {
        let remaining = col_options.len() - c + 1;
        let size = remaining.min(c);
        col_options[c - 1] = col_options[c - 1]
            && line[(c - size)..c]
                .chars()
                .zip(line[c..(c + size)].chars().rev())
                .all(|(a, b)| a == b);
    }
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
            405
        );
        assert_eq!(
            solve_file(read_to_string("inputs/day13.txt").unwrap()),
            37113
        );
    }
}
