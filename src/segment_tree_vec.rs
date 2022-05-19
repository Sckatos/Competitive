/// Segment tree.
///
/// Apply function like sum, minimum, etc... on a sub-array and answer queries with O(log(n)) complexity.
///
/// Can be updated.
/// # Example:
/// ```
/// let mut seg_tree = SegmentTree::from(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], |a, b| a + b);
///  assert_eq!(seg_tree.query(0, 10), 55);
///  seg_tree.update(0, 11);
///  assert_eq!(seg_tree.query(0, 10), 65);
/// ```
pub struct SegmentTree {
    tree: Vec<i64>,
    n: usize,
    func: fn(i64, i64) -> i64
}

impl SegmentTree {
    pub fn from(input: &[i64], func: fn(i64, i64) -> i64) -> Self {
        let n = input.len();
        let mut tree = vec![0; 2 * n];

        tree[n..(n + n)].copy_from_slice(input);
        for i in (1..n).rev() {
            tree[i] = func(tree[i << 1], tree[i << 1 | 1]);
        }

        Self { tree, n, func }
    }

    pub fn update(&mut self, mut pos: usize, val: i64) {
        pos += self.n;
        self.tree[pos] = val;
        while pos > 1 {
            self.tree[pos >> 1] = (self.func)(self.tree[pos], self.tree[pos ^ 1]);
            pos >>= 1;
        }
    }

    pub fn query(&self, mut left: usize, mut right: usize) -> i64 {
        let mut res = 0; // Careful! Initialization may vay

        left += self.n;
        right += self.n + 1;
        while left < right {
            if left & 1 == 1 {
                res = (self.func)(self.tree[left], res);
                left += 1;
            }
            if right & 1 == 1 {
                right -= 1;
                res = (self.func)(self.tree[right], res);
            }
            left >>= 1;
            right >>= 1;
        }
        res
    }
}
