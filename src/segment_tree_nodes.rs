/// Segment tree.
///
/// Apply function like sum, minimum, etc... on a sub-array and answer queries with O(log(n)) complexity.
///
/// Can be updated.
/// # Example:
/// ```
/// let mut seg_tree = SegmentTree::from(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], |a, b| a + b);
///  assert_eq!(seg_tree.query(0, 9), 55);
///  seg_tree.update(0, 11);
///  assert_eq!(seg_tree.query(0, 9), 65);
/// ```
pub struct SegmentTree<T> {
    node: Node<T>,
    func: fn(T, T) -> T
}

impl <T> SegmentTree<T> where T: Default + Copy {
    pub fn new(inputs: &[T], left: usize, right: usize, func: fn(T, T) -> T) -> Self {
        Self {
            node: Node::new(inputs, left, right, func),
            func
        }
    }

    pub fn from(inputs: &[T], func: fn(T, T) -> T) -> Self {
        Self {
            node: Node::from(inputs, func),
            func
        }
    }

    pub fn query(&self, left: usize, right: usize) -> T {
        self.node.query(left, right, self.func).unwrap()
    }

    pub fn update(&mut self, index: usize, value: T) {
       self.node.update(index, value, self.func);
    }
}

struct Node<T> {
    left_child: Option<Box<Node<T>>>,
    right_child: Option<Box<Node<T>>>,
    left: usize,
    right: usize,
    value: T
}

impl <T> Node<T> where T: Default + Copy {
    fn new(inputs: &[T], left: usize, right: usize, func: fn(T, T) -> T) -> Self {
        if left == right {
            Self {
                left,
                right,
                left_child: None,
                right_child: None,
                value: inputs[left],
            }
        } else {
            let middle = (left + right) / 2;
            let left_child = Some(Box::new(Node::new(inputs, left, middle, func)));
            let right_child = Some(Box::new(Node::new(inputs, middle + 1, right, func)));
            let value = func(left_child.as_ref().unwrap().value, right_child.as_ref().unwrap().value);

            Self {
                left,
                right,
                left_child,
                right_child,
                value,
            }
        }
    }

    fn from(inputs: &[T], func: fn(T, T) -> T) -> Self {
        Self::new(inputs, 0, inputs.len() - 1, func)
    }

    fn update(&mut self, index: usize, value: T, func: fn(T, T) -> T) {
        if self.left == self.right {
            self.value = value;
            return
        }
        if (self.left + self.right) / 2 >= index {
            self.left_child.as_mut().unwrap().update(index, value, func);
        } else {
            self.right_child.as_mut().unwrap().update(index, value, func);
        }
        self.value = func(self.left_child.as_ref().unwrap().value, self.right_child.as_ref().unwrap().value)
    }

    fn query(&self, left: usize, right: usize, func: fn(T, T) -> T) -> Option<T> {
        if self.left > right || self.right < left {
            return None;
        }
        if self.left >= left && self.right <= right {
            return Some(self.value);
        }
        let result = (
            self.left_child.as_ref()?.query(left, right, func),
            self.right_child.as_ref()?.query(left, right, func)
        );
        match result {
            (a, None) | (None, a)   => a,
                      _             => Some(func(result.0?, result.1?))
        }
    }
}
