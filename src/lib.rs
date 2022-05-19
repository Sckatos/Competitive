#![allow(dead_code)]

mod hungarian;
mod flow;
mod rmq;
mod segment_tree_node;
mod fenwick;

#[cfg(test)]
mod tests {
    use crate::fenwick::Fenwick;
    use crate::flow::max_flow;
    use crate::hungarian::hungarian;
    use crate::rmq::Rmq;
    use crate::segment_tree_node::SegmentTree;

    #[test]
    fn test_hungarian() {
        let costs = [
            vec![40, 60, 15],
            vec![25, 30, 45],
            vec![55, 30, 25]
        ];
        assert_eq!(hungarian(&costs), 70);
    }

    #[test]
    fn test_flow() {
        let graph = [
            vec![ 0, 16, 13, 0, 0, 0 ], vec![ 0, 0, 10, 12, 0, 0 ],
            vec![ 0, 4, 0, 0, 14, 0 ],  vec![ 0, 0, 9, 0, 0, 20 ],
            vec![ 0, 0, 0, 7, 0, 4 ],   vec![ 0, 0, 0, 0, 0, 0 ]
        ];
        assert_eq!(max_flow(&graph, 0, 5), 23);
    }

    #[test]
    fn test_rmq() {
        let rmq = Rmq::new(&[3, 1, 7, 2, 4, 3, 3],std::cmp::min);
        assert_eq!(rmq.query(0, 6), 1)
    }

    #[test]
    fn test_segment_tree() {
        let mut seg_tree = SegmentTree::from(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], |a, b| a + b);
        assert_eq!(seg_tree.query(0, 9), 55);
        seg_tree.update(0, 11);
        assert_eq!(seg_tree.query(0, 9), 65);
    }

    #[test]
    fn test_fenwick_tree() {
        let mut fenwick = Fenwick::from(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], |a, b| a + b);
        assert_eq!(fenwick.query(9), 55);
        fenwick.update(0, 11);
        assert_eq!(fenwick.query(9), 66);
    }
}