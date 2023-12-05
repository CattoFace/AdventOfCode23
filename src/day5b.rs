use std::{
    cmp,
    fs::File,
    io::BufRead,
    io::{BufReader, Lines},
};

pub fn solve_day() -> u64 {
    let file = File::open("inputs/day5.txt").unwrap();
    let ans = solve_file(file);
    assert_eq!(ans, 2008785);
    ans
}

fn line_modify_seeds(
    current_items: &[(u64, u64, u64)],
    mod_target: u64,
    mod_start: u64,
    mod_length: u64,
    curr_stage: u64,
) -> Vec<(u64, u64, u64)> {
    let mod_end = mod_start + mod_length;
    let mut new_items: Vec<(u64, u64, u64)> = Vec::with_capacity(current_items.len());
    current_items
        .iter()
        .flat_map(|(start, length, stage)| {
            let seed_end = *start + *length;
            let shared_start = cmp::max(*start, mod_start);
            let shared_end = cmp::min(seed_end, mod_end);
            let mut modified_values: Vec<(u64, u64, u64)> = Vec::new();
            if *stage == curr_stage {
                return vec![(*start, *length, *stage)];
            }
            if shared_start < shared_end {
                modified_values.push((
                    shared_start + mod_target - mod_start,
                    shared_end - shared_start,
                    curr_stage,
                ));
            }
            if *start < shared_start {
                modified_values.push((*start, cmp::min(*length, shared_start - *start), *stage));
            }
            if seed_end > shared_end {
                let new_start = cmp::max(*start, shared_end);
                modified_values.push((new_start, *length + *start - new_start, *stage));
            }
            modified_values
        })
        .for_each(|i| new_items.push(i));
    new_items
}
fn modify_seeds(
    lines: &mut Lines<BufReader<File>>,
    mut items: Vec<(u64, u64, u64)>,
    curr_stage: u64,
) -> Vec<(u64, u64, u64)> {
    lines
        .skip(1)
        .take_while(|s| {
            let unwrapped = s.as_ref().unwrap();
            !unwrapped.is_empty()
        })
        .for_each(|l| {
            let res: Vec<u64> = l
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect();
            items = line_modify_seeds(&items, res[0], res[1], res[2], curr_stage);
        });
    items
}
fn solve_file(file: File) -> u64 {
    let mut lines = BufReader::new(file).lines();
    let mut stage: Vec<(u64, u64, u64)> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .filter_map(|s| match s.parse::<u64>() {
            Ok(num) => Some(num),
            Err(_) => None,
        })
        .collect::<Vec<u64>>()
        .chunks(2)
        .map(|p| (p[0], p[1], 0))
        .collect();
    lines.by_ref().next();
    stage = modify_seeds(lines.by_ref(), stage, 1);
    stage = modify_seeds(lines.by_ref(), stage, 2);
    stage = modify_seeds(lines.by_ref(), stage, 3);
    stage = modify_seeds(lines.by_ref(), stage, 4);
    stage = modify_seeds(lines.by_ref(), stage, 5);
    stage = modify_seeds(lines.by_ref(), stage, 6);
    stage = modify_seeds(lines.by_ref(), stage, 7);
    *stage.iter().map(|(start, _, _)| start).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        assert_eq!(solve_file(File::open("inputs/day5_test.txt").unwrap()), 46);
        assert_eq!(solve_file(File::open("inputs/day5.txt").unwrap()), 2008785);
    }
}
