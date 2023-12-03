use std::{fs::File, io::BufRead, io::BufReader};

pub fn solve_day() {
    let file = File::open("inputs/dayX.txt").unwrap();
    assert_eq!(solve_file(file), 0);
    println!("pass")
}
fn solve_file(file: File) -> u32 {
    println!("Solving dayXx");
    let lines = BufReader::new(file).lines();
    // write solution
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        assert_eq!(solve_file(File::open("inputs/dayXx_test.txt").unwrap()), 0);
        assert_eq!(solve_file(File::open("inputs/dayX.txt").unwrap()), 0)
    }
}
