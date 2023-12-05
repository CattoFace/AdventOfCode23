use std::{
    fs::File,
    io::BufRead,
    io::{BufReader, Lines},
};

pub fn solve_day() -> u64 {
    let file = File::open("inputs/day5.txt").unwrap();
    let ans = solve_file(file);
    assert_eq!(ans, 88151870);
    ans
}

fn modify_seeds(lines: &mut Lines<BufReader<File>>, seeds: &mut [(u64, u64)], current_stage: u64) {
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
            seeds.iter_mut().for_each(|(seed, stage)| {
                if *stage < current_stage && *seed >= res[1] && *seed < res[1] + res[2] {
                    *seed += res[0];
                    *seed -= res[1];
                    *stage = current_stage;
                }
            })
        });
}
fn solve_file(file: File) -> u64 {
    let mut lines = BufReader::new(file).lines();
    let mut seeds: Vec<(u64, u64)> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .filter_map(|s| match s.parse::<u64>() {
            Ok(num) => Some((num, 0)),
            Err(_) => None,
        })
        .collect();
    lines.by_ref().next();
    modify_seeds(lines.by_ref(), &mut seeds, 1);
    modify_seeds(lines.by_ref(), &mut seeds, 2);
    modify_seeds(lines.by_ref(), &mut seeds, 3);
    modify_seeds(lines.by_ref(), &mut seeds, 4);
    modify_seeds(lines.by_ref(), &mut seeds, 5);
    modify_seeds(lines.by_ref(), &mut seeds, 6);
    modify_seeds(lines.by_ref(), &mut seeds, 7);
    *seeds.iter().map(|(seed, _)| seed).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        assert_eq!(solve_file(File::open("inputs/day5_test.txt").unwrap()), 35);
        assert_eq!(solve_file(File::open("inputs/day5.txt").unwrap()), 88151870);
    }
}
