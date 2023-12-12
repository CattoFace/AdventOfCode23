use std::fs::read_to_string;

pub fn solve_day() -> u32 {
    solve_file(read_to_string("inputs/dayX_test.txt").unwrap())
}
fn solve_file(text: String) -> u32 {
    // write solution
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        assert_eq!(
            solve_file(read_to_string("inputs/dayX_test.txt").unwrap()),
            0
        );
        assert_eq!(solve_file(read_to_string("inputs/dayX.txt").unwrap()), 0);
    }
}
