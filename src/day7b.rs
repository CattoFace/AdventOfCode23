use std::{cmp::Ordering, fs::File, io::BufRead, io::BufReader, iter::zip};

pub fn solve_day() -> u32 {
    let file = File::open("inputs/day7.txt").unwrap();
    let ans = solve_file(file);
    assert_eq!(ans, 250506580);
    ans
}
#[derive(Debug, PartialOrd, PartialEq, Ord, Eq)]
enum HandType {
    High,
    Pair,
    TwoPair,
    ThreeOf,
    House,
    FourOf,
    FiveOf,
}
#[derive(Debug)]
struct Hand {
    cards: [u8; 5],
    hand_type: HandType,
    bid: u32,
}
impl Eq for Hand {}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}
impl Hand {
    pub fn new(cards_str: &str, bid: u32) -> Self {
        let mut cards = [0u8; 5];
        cards_str
            .chars()
            .map(|c| match c {
                'A' => 12,
                'K' => 11,
                'Q' => 10,
                'T' => 9,
                '9' => 8,
                '8' => 7,
                '7' => 6,
                '6' => 5,
                '5' => 4,
                '4' => 3,
                '3' => 2,
                '2' => 1,
                'J' => 0,
                _ => 0,
            })
            .enumerate()
            .for_each(|(i, v)| cards[i] = v);
        let hand_type = rate_hand(&cards);
        Self {
            cards,
            hand_type,
            bid,
        }
    }
}

fn rate_hand(cards: &[u8]) -> HandType {
    let mut buckets = [0u8; 13];
    cards.iter().for_each(|c| buckets[*c as usize] += 1);
    let mut best: usize = 0;
    for (i, count) in buckets.iter().enumerate().skip(1) {
        if *count >= buckets[best] || best == 0 {
            best = i;
        }
    }
    // println!("{}", best);
    if best != 0 {
        buckets[best] += buckets[0];
    }
    let mut counts = [0u8; 6];
    buckets
        .iter()
        .skip(1)
        .for_each(|b| counts[*b as usize] += 1);
    // println!("{:?}", counts);
    match counts {
        [_, _, _, _, _, 1] => HandType::FiveOf,
        [_, _, _, _, 1, _] => HandType::FourOf,
        [_, _, 1, 1, _, _] => HandType::House,
        [_, _, _, 1, _, _] => HandType::ThreeOf,
        [_, _, 2, _, _, _] => HandType::TwoPair,
        [_, _, 1, _, _, _] => HandType::Pair,
        _ => HandType::High,
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type != other.hand_type {
            self.hand_type.cmp(&other.hand_type)
        } else {
            match zip(self.cards.iter(), other.cards.iter()).find_map(|(c1, c2)| {
                if c1 == c2 {
                    None
                } else {
                    Some(c1.cmp(c2))
                }
            }) {
                Some(o) => o,
                None => Ordering::Equal,
            }
        }
    }
}
fn solve_file(file: File) -> u32 {
    let lines = BufReader::new(file).lines();
    let mut hands: Vec<Hand> = lines
        .map(|l| {
            let unwrapped = l.unwrap();
            let (cards_str, bid_str) = unwrapped.split_once(' ').unwrap();
            let bid = bid_str.parse::<u32>().unwrap();
            Hand::new(cards_str, bid)
        })
        .collect();
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i as u32 + 1) * hand.bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        assert_eq!(
            solve_file(File::open("inputs/day7_test.txt").unwrap()),
            5905
        );
        assert_eq!(
            solve_file(File::open("inputs/day7.txt").unwrap()),
            250506580
        );
    }
}
