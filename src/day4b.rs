use std::{collections::BTreeSet, fs::File, io::BufRead, io::BufReader};

pub fn solve_day() -> u32 {
    let file = File::open("inputs/day4.txt").unwrap();
    let ans = solve_file(file);
    assert_eq!(ans, 10378710);
    ans
}

struct Game {
    winnings: u32,
    count: u32,
}

fn parse_game(game: &str) -> Game {
    let (_line_start, game) = game.split_once(':').unwrap();
    let (winning, available) = game.split_once('|').unwrap();
    let winning_nums: BTreeSet<u32> = winning
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let matching_count = available
        .split_whitespace()
        .filter(|s| winning_nums.contains(&s.parse::<u32>().unwrap()))
        .count();
    Game {
        winnings: matching_count as u32,
        count: 1,
    }
}
fn solve_file(file: File) -> u32 {
    let lines = BufReader::new(file).lines();
    let mut games: Vec<Game> = lines.map(|line| parse_game(&line.unwrap())).collect();
    let mut sum: u32 = 0;
    for index in 0..games.len() {
        sum += games[index].count;
        for i in 1usize..=games[index].winnings as usize {
            games[index + i].count += games[index].count;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        assert_eq!(solve_file(File::open("inputs/day4_test.txt").unwrap()), 30);
        assert_eq!(solve_file(File::open("inputs/day4.txt").unwrap()), 10378710);
    }
}
