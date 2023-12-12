use std::fs::read_to_string;

use itertools::Itertools;

pub fn solve_day() -> u32 {
    solve_file(read_to_string("inputs/day12.txt").unwrap())
}
fn count_options(line: &mut [u8], nums: &mut [u32], active: bool) -> u32 {
    // let s = std::str::from_utf8(line).unwrap();
    // dbg!(s, &nums, &active);
    if line.is_empty() {
        if nums.is_empty() || (nums.len() == 1 && nums[0] == 0) {
            return 1;
        } else {
            return 0;
        }
    }
    let c = line[0];
    match (c, active, nums[0]) {
        (b'.', true, 0) => count_options(
            &mut line[1..],
            if nums.len() > 1 { &mut nums[1..] } else { nums },
            false,
        ),
        (b'.', true, _) => 0,
        (b'.', false, _) => count_options(&mut line[1..], nums, false),
        (b'#', true, 0) => 0,
        (b'#', false, 0) => 0,
        (b'#', _, _) => {
            nums[0] -= 1;
            let ans = count_options(&mut line[1..], nums, true);
            nums[0] += 1;
            ans
        }
        (b'?', _, _) => {
            line[0] = b'.';
            let mut ans = count_options(line, nums, active);
            line[0] = b'#';
            ans += count_options(line, nums, active);
            line[0] = b'?';
            ans
        }
        _ => unreachable!(),
    }
}
fn solve_file(text: String) -> u32 {
    text.lines()
        .map(|l| {
            let (values_str, nums_str) = l.split_once(' ').unwrap();
            let mut nums = nums_str
                .split(',')
                .map(|s| s.parse::<u32>().unwrap())
                .collect_vec();
            let mut values_string = values_str.to_string();
            let values_arr = unsafe { values_string.as_bytes_mut() };
            count_options(values_arr, &mut nums, false)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        assert_eq!(
            solve_file(read_to_string("inputs/day12_test.txt").unwrap()),
            21
        );
        assert_eq!(solve_file(read_to_string("inputs/day12.txt").unwrap()), 0);
    }
}
