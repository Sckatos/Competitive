#![allow(dead_code)]

mod hungarian;
mod flow;

#[cfg(test)]
mod tests {
    use crate::flow::max_flow;
    use crate::hungarian::hungarian;
    use crate::palindrome;

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
}