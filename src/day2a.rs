use std::{fs::File, io::BufRead, io::BufReader};

pub fn solve_day() -> u32 {
    let file = File::open("inputs/day2.txt").unwrap();
    let ans = solve_file(file);
    assert_eq!(ans, 1931);
    ans
}
fn solve_file(file: File) -> u32 {
    let lines = BufReader::new(file).lines();
    let mut sum: u32 = 0;
    for (id, line) in lines.enumerate() {
        let l = match line {
            Ok(line) => line,
            Err(err) => panic!("Can't read line {}", err),
        };
        let split_line: Vec<&str> = l
            .split(&[' ', ',', ';'])
            .filter(|x| !x.is_empty())
            .collect();
        if split_line.chunks(2).all(|chunk| {
            let (count, color) = (chunk[0], chunk[1]);
            match color {
                "red" => count.parse::<u32>().unwrap() <= 12,
                "green" => count.parse::<u32>().unwrap() <= 13,
                "blue" => count.parse::<u32>().unwrap() <= 14,
                _ => count == "Game",
            }
        }) {
            sum += (id + 1) as u32;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        assert_eq!(solve_file(File::open("inputs/day2_test.txt").unwrap()), 8);
        assert_eq!(solve_file(File::open("inputs/day2.txt").unwrap()), 1931)
    }
}
