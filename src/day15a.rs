use std::fs::read_to_string;

pub fn solve_day() -> u32 {
    solve_file(read_to_string("inputs/day15.txt").unwrap())
}
fn solve_file(text: String) -> u32 {
    let text = text.as_bytes();
    let mut sum = 0u32;
    let mut hash = 0u8;
    text.iter().for_each(|&c| match c {
        // split hashes o , or line feed
        b',' | 10 => {
            sum += hash as u32;
            hash = 0;
        }
        // add to curr hash
        _ => {
            hash = hash.wrapping_add(c).wrapping_mul(17);
        }
    });
    // add last hash
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        assert_eq!(
            solve_file(read_to_string("inputs/day15_test.txt").unwrap()),
            1320
        );
        assert_eq!(
            solve_file(read_to_string("inputs/day15.txt").unwrap()),
            516469
        );
    }
}
