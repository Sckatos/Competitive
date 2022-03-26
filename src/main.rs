#![allow(dead_code)]
use std::str::FromStr;
fn line() -> String { let mut s = String::new(); std::io::stdin().read_line(&mut s).expect("Error reading from stdin"); s.trim().to_string() }
fn val<T: FromStr>() -> T { line().parse::<T>().ok().unwrap() }
fn vec<T: FromStr>() -> Vec<T> { line().split_whitespace().map(|x| x.parse::<T>().ok().unwrap()).collect() }
fn arr<T: FromStr, const N: usize>() -> [T; N] { vec().try_into().ok().unwrap() }
trait Join { fn join_to_string(&self) -> String; }
impl <T: ToString> Join for Vec<T> { fn join_to_string(&self) -> String { self.iter().map(T::to_string).collect::<Vec<_>>().join(" ") } }


fn main() {
   let k = vec![1, 2, 3];
    println!("{}", k.join_to_string());
}

fn solve() {

}