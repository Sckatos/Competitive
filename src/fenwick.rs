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
        let mut res = 0; // careful! initialisation may vary
        let mut pos = pos as i64 + 1;

        while pos > 0 {
            res = (self.func)(self.tree[pos as usize], res);
            pos -= pos & -pos;
        }

        res
    }
}
