mod input {
    use std::str::FromStr;

    pub fn read() -> String {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Error reading from stdin");
        input.trim().to_string()
    }

    pub fn read_val<T: FromStr>() -> T {
        read().parse::<T>().ok().unwrap()
    }

    pub fn read_vec<T: FromStr>() -> Vec<T> {
        read().split_whitespace().map(|x| x.parse::<T>().ok().unwrap()).collect()
    }

    pub fn read_array<T: FromStr, const N: usize>() -> [T; N] {
        read_vec().try_into().ok().unwrap()
    }
}