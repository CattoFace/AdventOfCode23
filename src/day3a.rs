use std::{cmp, collections::BTreeSet, fs::File, io::BufRead, io::BufReader};

pub fn solve_day() -> u32 {
    let file = File::open("inputs/day3.txt").unwrap();
    let ans = solve_file(file);
    assert_eq!(ans, 519444);
    ans
}

fn valid_location(
    l_index: usize,
    c_start: usize,
    num_len: usize,
    symbol_loc: &BTreeSet<(usize, usize)>,
) -> bool {
    if l_index >= 1 {
        for w in (cmp::max(1, c_start) - 1)..(c_start + num_len + 1) {
            if symbol_loc.contains(&(l_index - 1, w)) {
                return true;
            }
        }
    }
    if c_start >= 1 && symbol_loc.contains(&(l_index, c_start - 1)) {
        return true;
    }
    if symbol_loc.contains(&(l_index, c_start + num_len)) {
        return true;
    }
    for w in (cmp::max(1, c_start) - 1)..(c_start + num_len + 1) {
        if symbol_loc.contains(&(l_index + 1, w)) {
            return true;
        }
    }
    false
}
fn parse_to_end(to_parse: &str) -> Option<u32> {
    if to_parse.starts_with(|c: char| !c.is_ascii_digit()) {
        return None;
    }
    Some(
        to_parse
            .chars()
            .map_while(|c| c.to_digit(10))
            .fold(0, |res, digit| res * 10 + digit),
    )
}
fn solve_file(file: File) -> u32 {
    let lines: Vec<String> = BufReader::new(file).lines().map_while(Result::ok).collect();
    let symbol_locations: BTreeSet<(usize, usize)> = lines
        .iter()
        .enumerate()
        .flat_map(|(line_index, line)| {
            line.char_indices()
                .filter_map(move |(char_index, c)| match c {
                    '!' | '@' | '#' | '$' | '%' | '^' | '&' | '*' | '-' | '=' | '+' | '/' => {
                        Some((line_index, char_index))
                    }
                    _ => None,
                })
        })
        .collect();
    let mut sum: u32 = 0;
    lines.iter().enumerate().for_each(|(line_index, line)| {
        let mut i = 0;
        while i < line.len() {
            let shortened_line = &line[i..];
            let Some(res) = parse_to_end(shortened_line) else {
                i += 1;
                continue;
            };
            let num_len = format!("{}", res).len();
            if valid_location(line_index, i, num_len, &symbol_locations) {
                sum += res;
            }
            i += num_len;
        }
    });
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        assert_eq!(
            solve_file(File::open("inputs/day3_test.txt").unwrap()),
            4361
        );
        assert_eq!(solve_file(File::open("inputs/day3.txt").unwrap()), 519444)
    }
}
