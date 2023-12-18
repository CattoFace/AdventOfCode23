use std::{fs::File, io::BufRead, io::BufReader, iter::zip};

pub fn solve_day() -> u32 {
    let file = File::open("inputs/day6.txt").unwrap();
    let ans = solve_file(file);
    assert_eq!(ans, 6209190);
    ans
}
fn solve_race((time, distance): (u32, u32)) -> u32 {
    let inner = ((time * time - 4 * distance) as f32).sqrt();
    let t = time as f32;
    let f = (((t - inner) / 2f32) + 0.0001).ceil() as u32;
    let c = (((t + inner) / 2f32) - 0.0001).floor() as u32;
    c - f + 1
}
fn solve_file(file: File) -> u32 {
    let mut lines = BufReader::new(file).lines();
    let times_line = lines.by_ref().next().unwrap().unwrap();
    let (_, times_str) = times_line.split_once(':').unwrap();
    let times: Vec<u32> = times_str
        .split_ascii_whitespace()
        .filter_map(|s| match s.parse() {
            Ok(num) => Some(num),
            Err(_) => None,
        })
        .collect();
    let distances_line = lines.by_ref().next().unwrap().unwrap();
    let (_, distance_str) = distances_line.split_once(':').unwrap();
    let distances: Vec<u32> = distance_str
        .split_ascii_whitespace()
        .filter_map(|s| match s.parse() {
            Ok(num) => Some(num),
            Err(_) => None,
        })
        .collect();
    zip(times, distances).map(solve_race).product()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        assert_eq!(solve_file(File::open("inputs/day6_test.txt").unwrap()), 288);
        assert_eq!(solve_file(File::open("inputs/day6.txt").unwrap()), 6209190);
    }
}
