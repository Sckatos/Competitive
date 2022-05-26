// binary search

fn bound<T: Copy + PartialOrd>(vec: &[T], value: T, f: fn(T, T) -> bool) -> Option<usize> {
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
    if left < vec.len() { Some(left) } else { None }
}

trait VecOp<T> {
    fn lower_bound(&self, value: T) -> Option<usize>;

    fn upper_bound(&self, value: T) -> Option<usize>;
}

impl <T: ToString + PartialOrd + Copy> VecOp<T> for Vec<T> {
    fn lower_bound(&self, value: T) -> Option<usize> {
        bound(self, value, |a, b| a <= b)
    }

    fn upper_bound(&self, value: T) -> Option<usize> {
        bound(self, value, |a, b| a < b)
    }
}

// Vec join

trait VecStringJoin {
    fn join_to_string(&self) -> String;
}

impl <T> VecStringJoin for Vec<T> where T: ToString {
    fn join_to_string(&self) -> String {
        self.iter().map(T::to_string).collect::<Vec<_>>().join(" ")
    }
}

// primes
/// Retrieve all primitive numbers in [0, max] array
fn sieve(max: usize) -> Vec<usize> {
    let mut factors = vec![0usize; max + 1];
    let mut prime = vec![true; max + 1];
    let mut primes = vec![];

    prime[0] = false;
    prime[1] = false;
    for p in 2..=max {
        if prime[p] {
            factors[p] = p;
            primes.push(p);
        }
        for i in (p*p..=max).step_by(p) {
            if prime[i] {
                prime[i] = false;
                factors[i] = p;
            }
        }
    }
    primes
}

// GCD

pub fn gcd(mut u: u32, mut v: u32) -> u32 {
    use std::cmp::min;
    use std::mem::swap;

    if u == 0 {
        return v;
    } else if v == 0 {
        return u;
    }
    let i = u.trailing_zeros();  u >>= i;
    let j = v.trailing_zeros();  v >>= j;
    let k = min(i, j);

    loop {
        if u > v { swap(&mut u, &mut v); }
        v -= u;
        if v == 0 { return u << k }
        v >>= v.trailing_zeros();
    }
}

fn gcd2(mut a: i32, mut b: i32) -> i32 {
    let mut tmp;
    while b != 0 {
        tmp = a % b;
        a = b;
        b = tmp;
    }
    a
}

fn lcm(a: i32, b: i32) -> i32 {
    (a * b) / gcd(a, b)
}