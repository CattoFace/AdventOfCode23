use std::{collections::BTreeSet, fs::read_to_string};

use itertools::Itertools;

pub fn solve_day() -> usize {
    solve_file(read_to_string("inputs/day11.txt").unwrap(), 1000000)
}
fn solve_file(text: String, expansion: usize) -> usize {
    let width = text.lines().next().unwrap().len();
    let height = text.len() / (width + 1);
    // find rows and columns that expand
    let mut columns_counts = vec![0usize; width];
    let expanded_rows: BTreeSet<usize> = text
        .lines()
        .enumerate()
        .filter(|(_, l)| {
            l.chars()
                .enumerate()
                .inspect(|(j, c)| {
                    if *c == '.' {
                        columns_counts[*j] += 1;
                    }
                })
                .filter(|(_, c)| *c == '.')
                .count()
                == width
        })
        .map(|(i, _)| i)
        .collect();
    let expanded_columns: BTreeSet<usize> = columns_counts
        .iter()
        .enumerate()
        .filter(|(_, &count)| count == height)
        .map(|(i, _)| i)
        .collect();
    // register all galaxies after expansion
    let mut row_offset = 0usize;
    let galaxies: Vec<(usize, usize)> = text
        .lines()
        .enumerate()
        .flat_map(|(mut i, l)| {
            let mut column_offset = 0usize;
            if expanded_rows.contains(&i) {
                row_offset += expansion - 1;
            }
            i += row_offset;
            l.chars()
                .enumerate()
                .filter_map(|(j, c)| {
                    if expanded_columns.contains(&j) {
                        column_offset += expansion - 1;
                    }
                    if c == '#' {
                        Some((i, j + column_offset))
                    } else {
                        None
                    }
                })
                .collect_vec()
        })
        .collect();
    let mut sum = 0usize;
    for i in 0..galaxies.len() {
        let (x1, y1) = galaxies[i];
        ((i + 1)..galaxies.len()).for_each(|j| {
            let (x2, y2) = galaxies[j];
            sum += ((x1 as isize - x2 as isize).abs() + (y1 as isize - y2 as isize).abs()) as usize;
        });
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        assert_eq!(
            solve_file(read_to_string("inputs/day11_test.txt").unwrap(), 100),
            8410
        );
        assert_eq!(
            solve_file(read_to_string("inputs/day11.txt").unwrap(), 1000000),
            504715068438
        );
    }
}
