use std::{fs::File, io::BufRead, io::BufReader};

pub fn solve_day() -> u32 {
    let file = File::open("inputs/dayX.txt").unwrap();
    let ans = solve_file(file);
    ans
}
fn solve_file(file: File) -> u32 {
    let lines = BufReader::new(file).lines();
    // write solution
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        assert_eq!(solve_file(File::open("inputs/dayX_test.txt").unwrap()), 0);
        assert_eq!(solve_file(File::open("inputs/dayX.txt").unwrap()), 0);
    }
}
