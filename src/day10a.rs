use std::{fmt, fs::read_to_string};

pub fn solve_day() -> u32 {
    let text = read_to_string("inputs/day10.txt").unwrap();
    solve_file(text)
}

fn coords2index(x: usize, y: usize, width: usize) -> usize {
    y * width + x
}
impl PipeMap<'_> {
    #[allow(unused_assignments)]
    fn new(text: &str) -> PipeMap {
        let width = text.find('\n').unwrap() + 1;
        let height = text.len() / width;
        let start = text.find('S').unwrap();
        let map = text.as_bytes();
        let start_x = start % width;
        let start_y = start / width;
        let mut left = 0;
        let mut found_left = false;
        let mut left_direction = Direction::Up;
        if start_x != 0 {
            let addr = coords2index(start_x - 1, start_y, width);
            if "-LF".contains(map[addr] as char) {
                if !found_left {
                    left = addr;
                    found_left = true;
                    left_direction = Direction::Left;
                } else {
                    let right = addr;
                    return PipeMap {
                        values: map,
                        curr_right: right,
                        curr_left: left,
                        width,
                        dist: 1,
                        last_right: Direction::Left,
                        last_left: left_direction,
                    };
                }
            }
        }
        if start_x != width - 1 {
            let addr = coords2index(start_x + 1, start_y, width);
            if "-7J".contains(map[addr] as char) {
                if !found_left {
                    left = addr;
                    found_left = true;
                    left_direction = Direction::Right;
                } else {
                    let right = addr;
                    return PipeMap {
                        values: map,
                        curr_right: right,
                        curr_left: left,
                        width,
                        dist: 1,
                        last_right: Direction::Right,
                        last_left: left_direction,
                    };
                }
            }
        }
        if start_y != 0 {
            let addr = coords2index(start_x, start_y - 1, width);
            if "|F7".contains(map[addr] as char) {
                if !found_left {
                    left = addr;
                    found_left = true;
                    left_direction = Direction::Up;
                } else {
                    let right = addr;
                    return PipeMap {
                        values: map,
                        curr_right: right,
                        curr_left: left,
                        width,
                        dist: 1,
                        last_right: Direction::Up,
                        last_left: left_direction,
                    };
                }
            }
        }
        if start_y != height - 1 {
            let addr = coords2index(start_x, start_y + 1, width);
            if "|LJ".contains(map[addr] as char) {
                if !found_left {
                    left = addr;
                    found_left = true;
                    left_direction = Direction::Down;
                } else {
                    let right = addr;
                    return PipeMap {
                        values: map,
                        curr_right: right,
                        curr_left: left,
                        width,
                        dist: 1,
                        last_right: Direction::Down,
                        last_left: left_direction,
                    };
                }
            }
        }
        panic!("Did not find 2 neighbours");
    }

    fn advance(&mut self) -> Option<u32> {
        use Direction::*;
        let move_left = match (self.values[self.curr_left] as char, self.last_left) {
            ('|', Up) => Up,
            ('|', Down) => Down,
            ('-', Left) => Left,
            ('-', Right) => Right,
            ('F', Up) => Right,
            ('F', Left) => Down,
            ('L', Down) => Right,
            ('L', Left) => Up,
            ('7', Right) => Down,
            ('7', Up) => Left,
            ('J', Right) => Up,
            ('J', Down) => Left,
            _ => panic!("Impossible case"),
        };
        self.last_left = move_left;
        match move_left {
            Up => self.curr_left -= self.width,
            Down => self.curr_left += self.width,
            Left => self.curr_left -= 1,
            Right => self.curr_left += 1,
        }
        let move_right = match (self.values[self.curr_right] as char, self.last_right) {
            ('|', Up) => Up,
            ('|', Down) => Down,
            ('-', Left) => Left,
            ('-', Right) => Right,
            ('F', Up) => Right,
            ('F', Left) => Down,
            ('L', Down) => Right,
            ('L', Left) => Up,
            ('7', Right) => Down,
            ('7', Up) => Left,
            ('J', Right) => Up,
            ('J', Down) => Left,
            _ => panic!("Impossible case"),
        };
        self.last_right = move_right;
        match move_right {
            Up => self.curr_right -= self.width,
            Down => self.curr_right += self.width,
            Left => self.curr_right -= 1,
            Right => self.curr_right += 1,
        }
        self.dist += 1;
        if self.curr_left == self.curr_right {
            Some(self.dist)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl fmt::Debug for PipeMap<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "curr_right: {}, from_dir: {:?}\n curr_left: {}, from_dir: {:?}\n, dist: {}, width: {}",
            self.values[self.curr_right] as char,
            self.last_right,
            self.values[self.curr_left] as char,
            self.last_left,
            self.dist,
            self.width,
        )
    }
}
struct PipeMap<'a> {
    values: &'a [u8],
    curr_right: usize,
    curr_left: usize,
    width: usize,
    dist: u32,
    last_right: Direction,
    last_left: Direction,
}
fn solve_file(text: String) -> u32 {
    let mut pmap = PipeMap::new(&text);
    // println!("{:?}", pmap);
    loop {
        let dist = pmap.advance();
        if let Some(d) = dist {
            return d;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        assert_eq!(
            solve_file(read_to_string("inputs/day10_test.txt").unwrap()),
            8
        );
        assert_eq!(
            solve_file(read_to_string("inputs/day10.txt").unwrap()),
            6649
        );
    }
}
