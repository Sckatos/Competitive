#![allow(dead_code)]

use std::str::FromStr;

fn read_line() -> String {
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).expect("Error reading from stdin");
    input.trim().to_string()
}

fn read_val<T>() -> T where T: FromStr {
    read_line().parse::<T>().ok().unwrap()
}

fn read_vec<T>() -> Vec<T> where T: FromStr {
    read_line().split_whitespace().map(|x| x.parse::<T>().ok().unwrap()).collect()
}

fn read_arr<T, const N: usize>() -> [T; N] where T: FromStr {
    read_vec().try_into().ok().unwrap()
}

fn main() {

}


