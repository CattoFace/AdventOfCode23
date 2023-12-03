use std::cmp;
use std::{fs::File, io::BufRead, io::BufReader};

pub fn solve_day() {
    let file = File::open("inputs/day2.txt").unwrap();
    assert_eq!(solve_file(file), 83105);
    println!("pass")
}

fn solve_file(file: File) -> u32 {
    println!("Solving day2b");
    let lines = BufReader::new(file).lines();
    let mut sum: u32 = 0;
    for line in lines {
        let l = match line {
            Ok(line) => line,
            Err(err) => panic!("Can't read line {}", err),
        };
        let split_line: Vec<&str> = l
            .split(&[' ', ',', ';'])
            .filter(|x| !x.is_empty())
            .collect();
        let mut min_red: u32 = 0;
        let mut min_green: u32 = 0;
        let mut min_blue: u32 = 0;
        split_line.chunks(2).for_each(|chunk| {
            let (count, color) = (chunk[0], chunk[1]);
            match color {
                "red" => min_red = cmp::max(min_red, count.parse::<u32>().unwrap()),
                "green" => min_green = cmp::max(min_green, count.parse::<u32>().unwrap()),
                "blue" => min_blue = cmp::max(min_blue, count.parse::<u32>().unwrap()),
                &_ => (),
            }
        });
        sum += min_red * min_green * min_blue;
    }
    println!("{}", sum);
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        assert_eq!(
            solve_file(File::open("inputs/day2_test.txt").unwrap()),
            2286
        );
        assert_eq!(solve_file(File::open("inputs/day2.txt").unwrap()), 83105);
    }
}
