use std::fs::read_to_string;

pub fn solve_day() -> u16 {
    solve_file(read_to_string("inputs/day1.txt").unwrap())
}
fn solve_file(text: String) -> u16 {
    let map: [&[u8]; 9] = [
        b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
    ];
    let text = text.as_bytes();
    text.split(|&c| c == b'\n')
        .map(|mut line| {
            if line.is_empty() {
                return 0;
            }
            // println!("{}", str::from_utf8(line).unwrap());
            let first_num = (loop {
                if let Some(res) = (0..map.len())
                    .position(|n| line[0] - b'1' == n as u8 || line.starts_with(map[n]))
                {
                    break res;
                }
                line = &line[1..];
            } + 1) as u16
                * 10;
            let last_num = (loop {
                if let Some(res) = (0..map.len())
                    .position(|n| line[line.len() - 1] - b'1' == n as u8 || line.ends_with(map[n]))
                {
                    break res;
                }
                line = &line[..line.len() - 1];
            } + 1) as u16;
            first_num + last_num
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        assert_eq!(
            solve_file(read_to_string("inputs/day1b_test.txt").unwrap()),
            281
        );
        assert_eq!(
            solve_file(read_to_string("inputs/day1.txt").unwrap()),
            55291
        );
    }
}
