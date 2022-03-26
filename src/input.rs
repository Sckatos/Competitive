use std::str::FromStr;

fn line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Error reading from stdin");
    s.trim().to_string()

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
