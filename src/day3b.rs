use std::{cmp, collections::BTreeMap, fs::File, io::BufRead, io::BufReader};

pub fn solve_day() -> u32 {
    let file = File::open("inputs/day3.txt").unwrap();
    let ans = solve_file(file);
    assert_eq!(ans, 74528807);
    ans
}

fn try_add_to_cog(
    l_index: usize,
    c_start: usize,
    num_len: usize,
    symbol_loc: &mut BTreeMap<(usize, usize), (u32, u32)>,
    num: u32,
) {
    if l_index >= 1 {
        for w in (cmp::max(1, c_start) - 1)..(c_start + num_len + 1) {
            if let Some((prod, count)) = symbol_loc.get_mut(&(l_index - 1, w)) {
                if *count <= 2 {
                    *prod *= num;
                    *count += 1;
                } else {
                    symbol_loc.remove(&(l_index - 1, w));
                }
            }
        }
    }
    if c_start >= 1 {
        if let Some((prod, count)) = symbol_loc.get_mut(&(l_index, c_start - 1)) {
            if *count <= 2 {
                *prod *= num;
                *count += 1;
            } else {
                symbol_loc.remove(&(l_index, c_start - 1));
            }
        }
    }
    if let Some((prod, count)) = symbol_loc.get_mut(&(l_index, c_start + num_len)) {
        if *count <= 2 {
            *prod *= num;
            *count += 1;
        } else {
            symbol_loc.remove(&(l_index, c_start + num_len + 1));
        }
    }
    for w in (cmp::max(1, c_start) - 1)..(c_start + num_len + 1) {
        if let Some((prod, count)) = symbol_loc.get_mut(&(l_index + 1, w)) {
            if *count <= 2 {
                *prod *= num;
                *count += 1;
            } else {
                symbol_loc.remove(&(l_index + 1, w));
            }
        }
    }
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
    let mut symbol_locations: BTreeMap<(usize, usize), (u32, u32)> = lines
        .iter()
        .enumerate()
        .flat_map(|(line_index, line)| {
            line.char_indices()
                .filter_map(move |(char_index, c)| match c {
                    '*' => Some(((line_index, char_index), (1, 0))),
                    _ => None,
                })
        })
        .collect();
    lines.iter().enumerate().for_each(|(line_index, line)| {
        let mut i = 0;
        while i < line.len() {
            let shortened_line = &line[i..];
            let Some(res) = parse_to_end(shortened_line) else {
                i += 1;
                continue;
            };
            let num_len = format!("{}", res).len();
            try_add_to_cog(line_index, i, num_len, &mut symbol_locations, res);
            i += num_len;
        }
    });
    symbol_locations.values().fold(
        0,
        |sum, (prod, count)| if *count == 2 { sum + prod } else { sum },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        assert_eq!(
            solve_file(File::open("inputs/day3_test.txt").unwrap()),
            467835
        );
        assert_eq!(solve_file(File::open("inputs/day3.txt").unwrap()), 74528807)
    }
}
