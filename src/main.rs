#![allow(dead_code)]

use std::str::FromStr;

fn line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Error reading from stdin");
    input.trim().to_string()

}

fn val<T: FromStr>() -> T {
    line().parse::<T>().ok().unwrap()
}

fn vec<T: FromStr>() -> Vec<T> {
    line().split_whitespace().map(|x| x.parse::<T>().ok().unwrap()).collect()
}

fn arr<T: FromStr, const N: usize>() -> [T; N] {
    vec().try_into().ok().unwrap()
}

trait VecOp<T> {
    fn join_to_string(&self) -> String;

    fn lower_bound(&self, value: T) -> Option<(usize, T)>;

    fn upper_bound(&self, value: T) -> Option<(usize, T)>;
}

fn bound<T: Copy + PartialOrd>(v: &[T], value: T, f: fn(T, T) -> bool) -> Option<(usize, T)> {
    let mut low = 0usize;
    let len = v.len();
    let mut high = len;

    while low < high {
        let mid = low + (high - low)/2;
        if value <= v[mid] {
            high = mid
        } else {
            low = mid + 1
        }
    }
    if low == len { return None }
    if low < len && f(v[low], value) {
        low += 1;
        println!("ok")
    }
    if low == len { return None }

    Some((low, v[low]))
}

impl <T: ToString + PartialOrd + Copy> VecOp<T> for Vec<T> {
    fn join_to_string(&self) -> String {
        self.iter().map(T::to_string).collect::<Vec<_>>().join(" ")
    }

    fn lower_bound(&self, value: T) -> Option<(usize, T)> {
        bound(self, value, |a, b| a < b)
    }

    fn upper_bound(&self, value: T) -> Option<(usize, T)> {
        bound(self, value, |a, b| a <= b)
    }
}

fn main() {

}

fn solve() {

}