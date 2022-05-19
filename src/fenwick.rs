/// Fenwick Tree aka Binary Index Tree (BIT).
///
/// It is used to efficiently perform range queries, such as addition, multiplication and XOR...
/// on a sub-array and answer queries with O(log(n)) complexity.
///
/// Queries always start from index 0, to i; sub-array [0, i], 0 < i <= array length
///
/// Can be updated by applying the function on the previous element position with a new value.
/// # Example:
/// ```
/// let mut fenwick = Fenwick::from(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], |a, b| a + b);
///  assert_eq!(fenwick.query(0, 9), 55);
///  fenwick.update(0, 11); // We are performing a sum,
/// // so the update will be the sum on the position 0 with the new element (1 + 11)
///  assert_eq!(fenwick.query(0, 9), 66);
/// ```
pub struct Fenwick {
    pub tree: Vec<i64>,
    n: usize,
    func: fn(i64, i64) -> i64
}

impl Fenwick {
    pub fn from(input: &[i64], func: fn(i64, i64) -> i64) -> Self {
        let n = input.len();
        let mut tree = vec![0; n + 1];

        tree[1..(n + 1)].copy_from_slice(input);
        for i in 1..(n + 1) {
            let x = i as i64;
            let p = (x + (x & -x)) as usize;

            if p <= n {
                tree[p] = func(tree[p], tree[i])
            }
        }

        Self { tree, n, func }
    }

    pub fn update(&mut self, pos: usize, val: i64) {
        let mut pos = pos as i64 + 1;
        let n = self.n as i64;

        while pos <= n {
            self.tree[pos as usize] = (self.func)(self.tree[pos as usize], val);
            pos += pos & -pos;
        }
    }

    pub fn query(&self, pos: usize) -> i64 {
        let mut res = 0; // careful! initialization may vary
        let mut pos = pos as i64 + 1;

        while pos > 0 {
            res = (self.func)(self.tree[pos as usize], res);
            pos -= pos & -pos;
        }

        res
    }
}

/// Simple Fenwick (sum)
struct Bit {
    pub tree: Vec<i32>,
}

impl Bit {
    fn new(n: i32) -> Self {
        Bit { tree: vec![0; n as usize + 1] }
    }

    fn add(&mut self, mut pos: i32, val: i32) {
        pos += 1;
        while pos <= self.tree.len() as i32 {
            self.tree[pos as usize] += val;
            pos += pos & -pos;
        }
    }

    fn query(&self, mut pos: i32) -> i32 {
        let mut result = 0;
        pos += 1;
        while pos != 0 {
            result += self.tree[pos as usize];
            pos -= pos & -pos;
        }
        result
    }
}