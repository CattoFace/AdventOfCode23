use std::{collections::HashMap, fs::read_to_string};

use itertools::Itertools;
use rayon::{iter::ParallelIterator, str::ParallelString};

pub fn solve_day() -> u64 {
    solve_file(read_to_string("inputs/day12.txt").unwrap())
}
fn count_options(
    cache: &mut HashMap<(u8, u8, u8), u64>,
    mut line: &mut [u8],
    mut nums: &mut [u8],
    mut run: u8,
) -> u64 {
    let og_l = line.len() as u8;
    let og_n = nums.len() as u8;
    let og_r = run;
    if line.is_empty() {
        if nums.is_empty() || (nums.len() == 1 && nums[0] == run) {
            return 1;
        } else {
            return 0;
        }
    }
    let mut extra = 0;
    let mut out = loop {
        // let s = std::str::from_utf8(line).unwrap();
        // dbg!(s, &nums, &run);
        if let Some(ans) = cache.get(&(line.len() as u8, nums.len() as u8, run)) {
            return *ans;
        }
        let c = line[0];
        match c {
            b'.' => match run {
                0 => {}
                _ => {
                    if !nums.is_empty() && run == nums[0] {
                        nums = &mut nums[1..];
                        run = 0;
                    } else {
                        break 0;
                    }
                }
            },
            b'#' => {
                if !nums.is_empty() && run <= nums[0] {
                    run += 1;
                } else {
                    break 0;
                }
            }
            b'?' => {
                // calculate extra as .
                extra += match run {
                    0 => count_options(cache, &mut line[1..], nums, 0),
                    _ => {
                        if !nums.is_empty() && run == nums[0] {
                            count_options(cache, &mut line[1..], &mut nums[1..], 0)
                        } else {
                            0
                        }
                    }
                };
                // continue as #
                if !nums.is_empty() && run <= nums[0] {
                    run += 1;
                } else {
                    break 0;
                }
            }
            _ => unreachable!(),
        };
        line = &mut line[1..];
        if line.is_empty() {
            if nums.is_empty() || (nums.len() == 1 && nums[0] == run) {
                break 1;
            } else {
                break 0;
            }
        }
    };
    out += extra;
    cache.insert((og_l, og_n, og_r), out);
    out
}
fn solve_file(text: String) -> u64 {
    text.par_lines()
        .map(|l| {
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
                .map(|s| s.parse::<u8>().unwrap())
                .collect_vec();
            // let mut values_string = values_str.to_string();
            let values_arr = unsafe { new_values_str.as_bytes_mut() };
            let mut cache: HashMap<(u8, u8, u8), u64> =
                HashMap::with_capacity(new_nums_str.len() * values_arr.len() / 3);
            count_options(&mut cache, values_arr, &mut nums, 0)
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
