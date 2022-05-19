#![allow(dead_code)]

use std::str::FromStr;

pub fn line() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error reading from stdin");
    input
        .trim()
        .to_string()
}

fn val<T>() -> T
    where T: FromStr {
    line()
        .parse::<T>()
        .ok()
        .unwrap()
}

fn vec<T>() -> Vec<T>
    where T: FromStr {
    line()
        .split_whitespace()
        .map(|x|
            x.parse::<T>()
            .ok()
            .unwrap())
        .collect()
}

fn arr<T, const N: usize>() -> [T; N]
    where T: FromStr {
    vec()
        .try_into()
        .ok()
        .unwrap()
}





fn main() {
    let k: usize = val();
    for _ in 0..k { solve() }
}

fn solve() {

}