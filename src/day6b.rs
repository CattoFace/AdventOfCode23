use std::{fs::File, io::BufRead, io::BufReader};

pub fn solve_day() -> u64 {
    let file = File::open("inputs/day6.txt").unwrap();
    let ans = solve_file(file);
    assert_eq!(ans, 28545089);
    ans
}
fn solve_race((time, distance): (u64, u64)) -> u64 {
    let inner = ((time * time - 4 * distance) as f64).sqrt();
    let t = time as f64;
    let f = (((t - inner) / 2f64) - 0.0001).ceil();
    let c = (((t + inner) / 2f64) - 0.0001).floor();
    (c - f + 1f64) as u64
}
fn solve_file(file: File) -> u64 {
    let mut lines = BufReader::new(file).lines();
    let time_line = lines.by_ref().next().unwrap().unwrap();
    let (_, time_str) = time_line.split_once(':').unwrap();
    let time: u64 = time_str
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .collect::<String>()
        .parse()
        .unwrap();
    let distance_line = lines.by_ref().next().unwrap().unwrap();
    let (_, distance_str) = distance_line.split_once(':').unwrap();
    let distance: u64 = distance_str
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .collect::<String>()
        .parse()
        .unwrap();
    solve_race((time, distance))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        assert_eq!(
            solve_file(File::open("inputs/day6_test.txt").unwrap()),
            71503
        );
        assert_eq!(solve_file(File::open("inputs/day6.txt").unwrap()), 28545089);
    }
}
