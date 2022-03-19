fn bfs(graph: &[Vec<i32>], s: usize, t: usize, parent: &mut [Option<usize>]) -> bool {
    let n = graph.len();
    let mut visited = vec![false; n];
    let mut queue = std::collections::VecDeque::<usize>::new();

    queue.push_front(s);
    visited[s] = true;
    parent[s] = None;
    while !queue.is_empty() {
        let u = queue.pop_back().unwrap();
        for v in 0..n {
            if !visited[v] && graph[u][v] > 0 {
                parent[v] = Some(u);
                if v == t {
                    return true;
                }
                queue.push_front(v);
                visited[v] = true;
            }
        }

    }
    false
}

/// Max Flow Ford Fulkerson
///
/// Calculate the maximum flow in a flow network using BFS.
///
/// Finding augmenting path from the source (node 0) to the sink (node n-1) in a residual graph until
/// capacity saturation.
/// # Example:
/// ```
/// let graph = [
///             vec![ 0, 16, 13, 0, 0, 0 ], // Node 0 (source) connected to Node 1 with capacity 16
///                                         // and Node 2 with capacity 13
///             vec![ 0, 0, 10, 12, 0, 0 ], // Node 1 is connected to Node 2 and Node 3
///             vec![ 0, 4, 0, 0, 14, 0 ],  // Node 2 is connected to Node 1 and Node 4
///             vec![ 0, 0, 9, 0, 0, 20 ],  // ...
///             vec![ 0, 0, 0, 7, 0, 4 ],
///             vec![ 0, 0, 0, 0, 0, 0 ]
///     ];
/// assert_eq!(flow(&graph), 23);
/// ```
///
pub fn max_flow(graph: &[Vec<i32>], s: usize, t: usize) -> i32 {
    let n = graph.len();
    let mut r_graph = vec![vec![0; n]; n];
    let mut parent = vec![None; n];
    let mut max_flow = 0;

    for u in 0..n {
        for v in 0..n {
            r_graph[u][v] = graph[u][v];
        }
    }

    while bfs(&r_graph, s, t, &mut parent) {
        let mut path_flow = i32::MAX;
        let mut v = t;
        let mut u;

        while v != s {
            u = parent[v].unwrap();
            path_flow = std::cmp::min(path_flow, r_graph[u][v]);
            v = parent[v].unwrap();
        }

        v = t;
        while v != s {
            u = parent[v].unwrap();
            r_graph[u][v] -= path_flow;
            r_graph[v][u] += path_flow;
            v = parent[v].unwrap();
        }
        max_flow += path_flow;
    }
    max_flow
}