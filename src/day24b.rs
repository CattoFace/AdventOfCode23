extern crate blas_src;
use ndarray::prelude::*;
use ndarray_linalg::Solve;
use std::fs::read_to_string;
#[derive(Debug, Copy, Clone)]
struct Hail {
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}
fn atoi_neg(text: &[u8]) -> (i64, usize) {
    let mut sum = 0i64;
    let mut offset = 0usize;
    let neg = if text[0] == b'-' {
        offset += 1;
        true
    } else {
        false
    };
    while text[offset].is_ascii_digit() {
        sum = sum * 10 + (text[offset] - b'0') as i64;
        offset += 1;
    }
    (if neg { -sum } else { sum }, offset)
}
fn atoi(text: &[u8]) -> (i64, usize) {
    let mut sum = 0i64;
    let mut offset = 0usize;
    while text[offset].is_ascii_digit() {
        sum = sum * 10 + (text[offset] - b'0') as i64;
        offset += 1;
    }
    (sum, offset)
}
fn parse_text(text: String) -> Vec<Hail> {
    let mut text = text.as_bytes();
    let mut hails = Vec::<Hail>::new();
    for _ in 0..3 {
        if text.is_empty() {
            break;
        }
        let (x, offset) = atoi(text);
        let x = x as f64;
        text = &text[(offset + 2)..];
        let (y, offset) = atoi(text);
        let y = y as f64;
        text = &text[(offset + 2)..];
        let (z, offset) = atoi(text);
        let z = z as f64;
        text = &text[(offset + 3)..];
        let (vx, offset) = atoi_neg(text);
        let vx = vx as f64;
        text = &text[(offset + 2)..];
        let (vy, offset) = atoi_neg(text);
        let vy = vy as f64;
        text = &text[(offset + 2)..];
        let (vz, offset) = atoi_neg(text);
        let vz = vz as f64;
        text = &text[(offset + 1)..];
        // println!("{},{},{},{},{},{}", x, y, z, vx, vy, vz);
        hails.push(Hail {
            x,
            y,
            z,
            vx,
            vy,
            vz,
        });
    }
    hails
}
pub fn solve_day() -> i64 {
    solve_file(read_to_string("inputs/day24_test.txt").unwrap())
}
fn solve_file(text: String) -> i64 {
    let hails = parse_text(text);
    // println!("{:?}", lines);
    find_rock_throw(hails) as i64
}
fn find_rock_throw(hails: Vec<Hail>) -> f64 {
    let hail1 = hails[0];
    let hail2 = hails[1];
    let hail3 = hails[2];
    let a: Array2<f64> = array![
        [
            -(hail1.vy - hail2.vy),
            hail1.vx - hail2.vx,
            0.,
            hail1.y - hail2.y,
            -(hail1.x - hail2.x),
            0.
        ],
        [
            -(hail1.vy - hail3.vy),
            hail1.vx - hail3.vx,
            0.,
            hail1.y - hail3.y,
            -(hail1.x - hail3.x),
            0.
        ],
        [
            0.,
            -(hail1.vz - hail2.vz),
            hail1.vy - hail2.vy,
            0.,
            hail1.z - hail2.z,
            -(hail1.y - hail2.y)
        ],
        [
            0.,
            -(hail1.vz - hail3.vz),
            hail1.vy - hail3.vy,
            0.,
            hail1.z - hail3.z,
            -(hail1.y - hail3.y)
        ],
        [
            -(hail1.vz - hail2.vz),
            0.,
            hail1.vx - hail2.vx,
            hail1.z - hail2.z,
            0.,
            -(hail1.x - hail2.x)
        ],
        [
            -(hail1.vz - hail3.vz),
            0.,
            hail1.vx - hail3.vx,
            hail1.z - hail3.z,
            0.,
            -(hail1.x - hail3.x)
        ]
    ];
    let b: Array1<f64> = array![
        (hail1.y * hail1.vx - hail2.y * hail2.vx) - (hail1.x * hail1.vy - hail2.x * hail2.vy),
        (hail1.y * hail1.vx - hail3.y * hail3.vx) - (hail1.x * hail1.vy - hail3.x * hail3.vy),
        (hail1.z * hail1.vy - hail2.z * hail2.vy) - (hail1.y * hail1.vz - hail2.y * hail2.vz),
        (hail1.z * hail1.vy - hail3.z * hail3.vy) - (hail1.y * hail1.vz - hail3.y * hail3.vz),
        (hail1.z * hail1.vx - hail2.z * hail2.vx) - (hail1.x * hail1.vz - hail2.x * hail2.vz),
        (hail1.z * hail1.vx - hail3.z * hail3.vx) - (hail1.x * hail1.vz - hail3.x * hail3.vz)
    ];
    let coeff = a.solve_into(b).unwrap();
    // println!("{:?}", &coeff.slice(s![..3]));
    coeff.slice(s![..3]).sum().round()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_test() {
        assert_eq!(
            solve_file(read_to_string("inputs/day24_test.txt").unwrap()),
            47
        );
        assert_eq!(
            solve_file(read_to_string("inputs/day24.txt").unwrap()),
            716599937560103
        );
    }
}
