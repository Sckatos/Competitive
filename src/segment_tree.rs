pub struct SegmentTree<T> {
    left_child: Option<Box<SegmentTree<T>>>,
    right_child: Option<Box<SegmentTree<T>>>,
    left: usize,
    right: usize,
    value: T,
    func: fn(T, T) -> T
}

impl <T> SegmentTree<T> where T: Default + Copy {
    pub fn new(inputs: &[T], left: usize, right: usize, func: fn(T, T) -> T) -> Self {
        if left == right {
            Self {
                left,
                right,
                left_child: None,
                right_child: None,
                value: inputs[left],
                func
            }
        } else {
            let middle = (left + right) / 2;
            let left_child = Some(Box::new(SegmentTree::new(inputs, left, middle, func)));
            let right_child = Some(Box::new(SegmentTree::new(inputs, middle + 1, right, func)));
            let value = func(left_child.as_ref().unwrap().value, right_child.as_ref().unwrap().value);

            Self {
                left,
                right,
                left_child,
                right_child,
                value,
                func
            }
        }
    }

    pub fn from(inputs: &[T], func: fn(T, T) -> T) -> Self {
        Self::new(inputs, 0, inputs.len() - 1, func)
    }

    pub fn query(&self, left: usize, right: usize) -> T {
        self._query(left, right).unwrap()
    }

    pub fn update(&mut self, index: usize, value: T) {
        if self.left == self.right {
            self.value = value;
            return
        }
        if (self.left + self.right) / 2 >= index {
            self.left_child.as_mut().unwrap().update(index, value);
        } else {
            self.right_child.as_mut().unwrap().update(index, value);
        }
        self.value = (self.func)(self.left_child.as_ref().unwrap().value, self.right_child.as_ref().unwrap().value)
    }

    fn _query(&self, left: usize, right: usize) -> Option<T> {
        if self.left > right || self.right < left {
            return None;
        }
        if self.left >= left && self.right <= right {
            return Some(self.value);
        }
        let result = (
            self.left_child.as_ref()?._query(left, right),
            self.right_child.as_ref()?._query(left, right)
        );
        match result {
            (a, None) | (None, a)   => a,
                      _             => Some((self.func)(result.0?, result.1?))
        }
    }
}
