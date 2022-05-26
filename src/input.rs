mod io {
    use std::str::{FromStr, SplitWhitespace};

    pub fn read_from_stdin() -> String {
        use std::io::Read;
        let mut input = String::new();

        std::io::stdin()
            .read_to_string(&mut input)
            .expect("Error reading from stdin");
        input
            .trim()
            .to_string()
    }

    pub struct Input<'a> {
        values: SplitWhitespace<'a>
    }

    impl <'a> Input<'a> {
        pub fn from(input: &'a str) -> Self {
            Self { values: input.split_whitespace() }
        }

        pub fn next<T>(&mut self) -> T where T: FromStr {
            self
                .values
                .next()
                .unwrap()
                .parse::<T>()
                .ok()
                .unwrap()
        }

        pub fn vec<T>(&mut self, n: usize) -> Vec<T> where T: FromStr {
            (0..n)
                .map(|_| self.next())
                .collect()
        }
    }
}