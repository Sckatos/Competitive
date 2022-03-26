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

fn bound<T: Copy + PartialOrd>(vec: &[T], value: T, f: fn(T, T) -> bool) -> Option<(usize, T)> {
    let mut left = 0usize;
    let mut right = vec.len();

    while left < right {
        let mid = (right + left) / 2;
        if f(value, vec[mid]) {
            right = mid
        } else {
            left = mid + 1
        }
    }
    if left < vec.len() { Some((left, vec[left])) } else { None }
}

impl <T: ToString + PartialOrd + Copy> VecOp<T> for Vec<T> {
    fn join_to_string(&self) -> String {
        self.iter().map(T::to_string).collect::<Vec<_>>().join(" ")
    }

    fn lower_bound(&self, value: T) -> Option<(usize, T)> {
        bound(self, value, |a, b| a <= b)
    }

    fn upper_bound(&self, value: T) -> Option<(usize, T)> {
        bound(self, value, |a, b| a < b)
    }
}

fn main() {
    let k = vec![1, 1, 2, 3, 4, 4, 5, 5, 8];

    for i in 0..10 {
        println!("{i} lower: {:?}", k.lower_bound(i));
        println!("{i} upper: {:?}", k.upper_bound(i));
    }

}

fn solve() {

}