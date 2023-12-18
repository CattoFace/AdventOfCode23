use std::{fmt, fs::read_to_string};

pub fn solve_day() -> u32 {
    solve_file(read_to_string("inputs/day10.txt").unwrap())
}

fn coords2index(x: usize, y: usize, width: usize) -> usize {
    y * width + x
}
fn find_start_val(dir1: Direction, dir2: Direction) -> u8 {
    use Direction::*;
    let val = match (dir1, dir2) {
        (Up, Down) => b'|',
        (Up, Left) => b'J',
        (Up, Right) => b'L',
        (Down, Up) => b'|',
        (Down, Left) => b'7',
        (Down, Right) => b'F',
        (Left, Up) => b'J',
        (Left, Down) => b'F',
        (Left, Right) => b'-',
        (Right, Up) => b'L',
        (Right, Down) => b'F',
        (Right, Left) => b'-',
        _ => panic!("Impossible case"),
    };
    val - 1u8
}
impl PipeMap<'_> {
    #[allow(unused_assignments)]
    fn new(text: &mut str) -> PipeMap {
        let width = text.find('\n').unwrap() + 1;
        let height = text.len() / width;
        let start = text.find('S').unwrap();
        let map = unsafe { text.as_bytes_mut() };
        map[start] += 1;
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
                    map[start] = find_start_val(left_direction, Direction::Left);
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
                    map[start] = find_start_val(left_direction, Direction::Right);
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
                    map[start] = find_start_val(left_direction, Direction::Up);
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
                    map[start] = find_start_val(left_direction, Direction::Down);
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
        self.values[self.curr_left] -= 1;
        // self.values[self.curr_left] = match self.last_left {
        //     Up | Down => b'*',
        //     Left | Right => b'%',
        // };
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
        self.values[self.curr_right] -= 1;
        // self.values[self.curr_right] = match self.last_right {
        // Up | Down => b'*',
        // Left | Right => b'%',
        // };
        self.last_right = move_right;
        match move_right {
            Up => self.curr_right -= self.width,
            Down => self.curr_right += self.width,
            Left => self.curr_right -= 1,
            Right => self.curr_right += 1,
        }
        self.dist += 1;
        if self.curr_left == self.curr_right {
            self.values[self.curr_right] -= 1;
            // self.values[self.curr_right] = match (self.last_right, self.last_left) {
            //     (Left, _) | (Right, _) | (_, Left) | (_, Right) => b'%',
            //     _ => b'*',
            // };
            Some(self.dist)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
    values: &'a mut [u8],
    curr_right: usize,
    curr_left: usize,
    width: usize,
    dist: u32,
    last_right: Direction,
    last_left: Direction,
}
fn solve_file(mut text: String) -> u32 {
    let mut pmap = PipeMap::new(&mut text);
    while pmap.advance().is_none() {}
    // println!(
    //     "{}",
    //     String::from_utf8(
    //         pmap.values
    //             .iter()
    //             .map(|c| {
    //                 if *c == b'\n' || *c == b'.' {
    //                     *c
    //                 } else {
    //                     c + 1
    //                 }
    //             })
    //             .collect::<Vec<u8>>()
    //     )
    //     .unwrap()
    // );
    //
    pmap.values
        .iter()
        .fold((0u32, false), |(count, inside): (u32, bool), &c: &u8| {
            // println!("{},{}", v as char, inside);
            match c + 1 {
                b'F' | b'7' | b'|' => (count, !inside),
                b'L' | b'J' | b'-' => (count, inside),
                _ => (count + inside as u32, inside),
            }
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        assert_eq!(
            solve_file(read_to_string("inputs/day10b_test.txt").unwrap()),
            10
        );
        assert_eq!(solve_file(read_to_string("inputs/day10.txt").unwrap()), 601);
    }
}
