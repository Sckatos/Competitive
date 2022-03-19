pub struct Rmq<T> {
    pub values: Vec<Vec<T>>,
    func: fn(T, T) -> T
}

impl <T> Rmq<T> where T: Default + Copy {
    pub fn new(inputs: &[T], func: fn(T, T) -> T) -> Self {
        let n = inputs.len();
        let log_n = log_n(n);
        let mut values = vec![vec![T::default(); n]; log_n + 1];

        values[0].clone_from_slice(inputs);
        for i in 1..=log_n {
            for j in 0..n - (1 << i) + 1 {
                values[i][j] = func(values[i - 1][j], values[i - 1][j + (1 << (i - 1))]);
            }
        }

        Rmq { values, func }
    }

    pub fn query(&self, left: usize, right: usize) -> T {
        let k = log_n(right - left + 1);
        (self.func)(self.values[k][left], self.values[k][right + 1 - (1 << k)])
    }
}

pub fn log_n(n: usize) -> usize {
    std::mem::size_of::<usize>() * 8 - n.leading_zeros() as usize - 1
}