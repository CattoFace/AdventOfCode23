use bitvec::prelude::*;
use colored::Colorize;
use std::fs::read_to_string;
pub fn solve_day() -> u16 {
    solve_file(read_to_string("inputs/day17.txt").unwrap())
}
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Axis {
    Horizontal,
    Vertical,
}

#[derive(PartialEq, Eq, Debug)]
struct DistrictMove {
    x: u8,
    y: u8,
    axis: Axis,
}

#[allow(dead_code)]
fn print_grid(text: &[u8], index: usize) {
    println!();
    text.iter().enumerate().for_each(|(i, &c)| {
        if i == index {
            print!("{}", "O".red());
        } else {
            print!("{}", c as char);
        }
    });
    println!();
}
#[derive(Debug)]
struct MyBucketQueue {
    start: u16,
    heap: Vec<Vec<DistrictMove>>,
}
impl MyBucketQueue {
    fn new() -> MyBucketQueue {
        MyBucketQueue {
            heap: Vec::new(),
            start: 0,
        }
    }
    fn push(&mut self, district: DistrictMove, cost: u16) {
        if (self.heap.len() as u16) < cost + 1 {
            self.heap.resize_with(cost as usize + 1, Vec::new)
        }
        self.start = self.start.min(cost);
        self.heap[cost as usize].push(district);
    }
    fn pop(&mut self) -> (DistrictMove, u16) {
        if self.heap[self.start as usize].is_empty() {
            self.start += self.heap[self.start as usize..]
                .iter()
                .position(|v| !v.is_empty())
                .unwrap() as u16;
        }
        (self.heap[self.start as usize].pop().unwrap(), self.start)
    }
}

fn solve_file(text: String) -> u16 {
    use Axis::*;
    let text = text.as_bytes();
    let width = text.iter().position(|&c| c == b'\n').unwrap() as u8;
    let height = (text.len() as u16 / (width + 1) as u16) as u8;
    let mut visited = bitvec![0;text.len()*2]; // 2 axis
                                               // let mut queue = BinaryHeap::<DistrictMove>::new();
    let mut queue = MyBucketQueue::new();
    queue.push(
        DistrictMove {
            x: 0,
            y: 0,
            axis: Vertical,
        },
        0,
    );
    queue.push(
        DistrictMove {
            x: 0,
            y: 0,
            axis: Horizontal,
        },
        0,
    );
    loop {
        let (curr_district, cost) = queue.pop();
        let index = curr_district.x as usize + curr_district.y as usize * (width + 1) as usize;
        let visited_index = index + curr_district.axis as usize * text.len();
        // print_grid(text, index);
        // println!("{:?}", queue);
        // let mut s = String::new();
        // std::io::stdin().read_line(&mut s);
        if visited[visited_index] {
            continue;
        }
        visited.set(visited_index, true);
        if curr_district.x == width - 1 && curr_district.y == height - 1 {
            return cost;
        }
        if !matches!(curr_district.axis, Horizontal) {
            let mut cost_sum = cost;
            for jump in 1u8..=3u8 {
                if curr_district.y >= jump {
                    cost_sum += (text[index - jump as usize * (width as usize + 1)] - b'0') as u16;
                    queue.push(
                        DistrictMove {
                            x: curr_district.x,
                            y: curr_district.y - jump,
                            axis: Horizontal,
                        },
                        cost_sum,
                    );
                } else {
                    break;
                }
            }
            let mut cost_sum = cost;
            for jump in 1u8..=3u8 {
                if curr_district.y + jump < height {
                    cost_sum += (text[index + jump as usize * (width as usize + 1)] - b'0') as u16;
                    queue.push(
                        DistrictMove {
                            x: curr_district.x,
                            y: curr_district.y + jump,
                            axis: Horizontal,
                        },
                        cost_sum,
                    );
                } else {
                    break;
                }
            }
        }
        if !matches!(curr_district.axis, Vertical) {
            let mut cost_sum = cost;
            for jump in 1u8..=3u8 {
                if curr_district.x >= jump {
                    cost_sum += (text[index - jump as usize] - b'0') as u16;
                    queue.push(
                        DistrictMove {
                            x: curr_district.x - jump,
                            y: curr_district.y,
                            axis: Vertical,
                        },
                        cost_sum,
                    );
                } else {
                    break;
                }
            }
            let mut cost_sum = cost;
            for jump in 1u8..=3u8 {
                if curr_district.x + jump < width {
                    cost_sum += (text[index + jump as usize] - b'0') as u16;
                    queue.push(
                        DistrictMove {
                            x: curr_district.x + jump,
                            y: curr_district.y,
                            axis: Vertical,
                        },
                        cost_sum,
                    );
                } else {
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        assert_eq!(
            solve_file(read_to_string("inputs/day17_test.txt").unwrap()),
            102
        );
        assert_eq!(solve_file(read_to_string("inputs/day17.txt").unwrap()), 936);
    }
}
