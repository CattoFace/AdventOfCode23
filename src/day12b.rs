use std::{collections::HashMap, fs::read_to_string};

use itertools::Itertools;

pub fn solve_day() -> u64 {
    solve_file(read_to_string("inputs/day12.txt").unwrap())
}
fn count_options(
    cache: &mut HashMap<(usize, usize, u8, u64, bool), u64>,
    line: &mut [u8],
    nums: &mut [u64],
    active: bool,
) -> u64 {
    // let s = std::str::from_utf8(line).unwrap();
    // dbg!(s, &nums, &active);
    if line.is_empty() {
        if nums.is_empty() || (nums.len() == 1 && nums[0] == 0) {
            return 1;
        } else {
            return 0;
        }
    }
    if let Some(ans) = cache.get(&(line.len(), nums.len(), line[0], nums[0], active)) {
        return *ans;
    }
    let c = line[0];
    let out = match (c, active, nums[0]) {
        (b'.', true, 0) => count_options(
            cache,
            &mut line[1..],
            if nums.len() > 1 { &mut nums[1..] } else { nums },
            false,
        ),
        (b'.', true, _) => 0,
        (b'.', false, _) => count_options(cache, &mut line[1..], nums, false),
        (b'#', true, 0) => 0,
        (b'#', false, 0) => 0,
        (b'#', _, _) => {
            nums[0] -= 1;
            let ans = count_options(cache, &mut line[1..], nums, true);
            nums[0] += 1;
            ans
        }
        (b'?', _, _) => {
            line[0] = b'.';
            let mut ans = count_options(cache, line, nums, active);
            line[0] = b'#';
            ans += count_options(cache, line, nums, active);
            line[0] = b'?';
            ans
        }
        _ => unreachable!(),
    };
    cache.insert((line.len(), nums.len(), line[0], nums[0], active), out);
    out
}
fn solve_file(text: String) -> u64 {
    text.lines()
        .map(|l| {
            let mut cache: HashMap<(usize, usize, u8, u64, bool), u64> = HashMap::new();
            let (values_str, nums_str) = l.split_once(' ').unwrap();
            let mut new_values_str = String::from(values_str);
            new_values_str.push('?');
            new_values_str.push_str(values_str);
            new_values_str.push('?');
            new_values_str.push_str(values_str);
            new_values_str.push('?');
            new_values_str.push_str(values_str);
            new_values_str.push('?');
            new_values_str.push_str(values_str);
            let mut new_nums_str = String::from(nums_str);
            new_nums_str.push(',');
            new_nums_str.push_str(nums_str);
            new_nums_str.push(',');
            new_nums_str.push_str(nums_str);
            new_nums_str.push(',');
            new_nums_str.push_str(nums_str);
            new_nums_str.push(',');
            new_nums_str.push_str(nums_str);
            let mut nums = new_nums_str
                .split(',')
                .map(|s| s.parse::<u64>().unwrap())
                .collect_vec();
            // let mut values_string = values_str.to_string();
            let values_arr = unsafe { new_values_str.as_bytes_mut() };
            count_options(&mut cache, values_arr, &mut nums, false)
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
            525152
        );
        assert_eq!(
            solve_file(read_to_string("inputs/day12.txt").unwrap()),
            4964259839627
        );
    }
}
