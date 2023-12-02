use std::{fs::File, io::BufRead, io::BufReader};

pub fn main() {
    let file = File::open("input.txt").unwrap();
    let lines = BufReader::new(file).lines();
}
