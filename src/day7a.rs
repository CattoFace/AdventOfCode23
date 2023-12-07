use std::{cmp::Ordering, fs::File, io::BufRead, io::BufReader, iter::zip};

pub fn solve_day() -> u32 {
    let file = File::open("inputs/day7.txt").unwrap();
    let ans = solve_file(file);
    assert_eq!(ans, 250058342);
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
                'J' => 9,
                'T' => 8,
                '9' => 7,
                '8' => 6,
                '7' => 5,
                '6' => 4,
                '5' => 3,
                '4' => 2,
                '3' => 1,
                '2' => 0,
                _ => 13,
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
    let mut curr_type = HandType::High;
    let mut pairs = 0;
    cards.iter().for_each(|c| {
        let i = *c as usize;
        buckets[i] += 1;
        match buckets[i] {
            5 => curr_type = HandType::FiveOf,
            4 => {
                if curr_type < HandType::FourOf {
                    curr_type = HandType::FourOf
                }
            }
            3 => {
                if pairs == 2 {
                    curr_type = HandType::House;
                } else if curr_type < HandType::ThreeOf {
                    curr_type = HandType::ThreeOf
                }
            }
            2 => {
                pairs += 1;
                if curr_type < HandType::TwoPair {
                    curr_type = if pairs == 1 {
                        HandType::Pair
                    } else {
                        HandType::TwoPair
                    }
                } else if curr_type == HandType::ThreeOf {
                    curr_type = HandType::House;
                }
            }
            _ => (),
        }
    });
    curr_type
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
    // for h in hands.iter() {
    // println!("{:?}", h);
    // }
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
            6440
        );
        assert_eq!(
            solve_file(File::open("inputs/day7.txt").unwrap()),
            250058342
        );
    }
}
