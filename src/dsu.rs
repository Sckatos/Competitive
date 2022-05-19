use std::ops::Index;

/// Disjoint Set Union
///
/// A union-find data structure that keeps track of elements partitioned in non overlapping subsets.
pub struct Dsu {
    parent: Vec<usize>,
    rank: Vec<i32>
}

impl Dsu {
    pub fn new(n: usize) -> Self {
        let mut parent = vec![0usize; n];
        let mut rank = vec![0; n];

        for i in 0..n {
            parent[i] = i;
            rank[i] = 0;
        }

        Self { parent, rank }
    }

    /// Find the parent (leader) of the node
    pub fn find(&self, mut node: usize) -> usize {
        while node != self.parent[node] {
            node = self.parent[node];
        }
        node
    }

    /// Join 2 sets together, the set with higher rank becomes the leader
    pub fn union(&mut self, mut u: usize, mut v: usize) {
        use std::mem::swap;
        u = self.find(u);
        v = self.find(v);

        if u != v {
            if self.rank[u] < self.rank[v] {
                swap(&mut u, &mut v)
            }
            self.parent[v] = u;
            if self.rank[u] == self.rank[v] {
                self.rank[u] += 1;
            }
        }
    }
}

/// Sugar syntax dsu.find(node) => dsu[node]
impl Index<usize> for Dsu {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.parent[self.find(index)]
    }
}

/// Disjoint Set Union
///
/// Naive version of DSU
pub struct NDsu {
    parent: Vec<usize>
}

impl NDsu {
    pub fn new(n: usize) -> Self {
        let mut parent = vec![0; n];

        for i in 0..n {
            parent[i] = i;
        }

        Self { parent }
    }

    pub fn find(&self, mut node: usize) -> usize {
        while node != self.parent[node] {
            node = self.parent[node];
        }

        node
    }

    pub fn union(&mut self, mut node: usize, mut leader: usize) {
        node = self.find(node);
        leader = self.find(leader);
        self.parent[node] = leader;
    }
}
