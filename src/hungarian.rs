/// Hungarian algorithm.
///
/// Solves the assignment problem using a bipartite graph N edges connected to M edges with N <= M.
///
/// # Example:
/// Lets assume you wanna assign 3 workers to 3 jobs.
///
/// We wanna assign a worker for each job while minimizing the overall cost.
/// ```
/// let costs = [
///         vec![40, 60, 15],   // worker 1 can do job 1 for 40$, job2 for 60$ and job3 for 15$
///         vec![25, 30, 45],   // worker 2
///         vec![55, 30, 25]    // worker 3
/// ];
/// assert_eq!(hungarian(&costs), 70);
/// ```
/// # Note:
/// Values start from 0.
///
pub fn hungarian(cost: &[Vec<i64>]) -> i64 {
    let n = cost.len();
    let m = cost[0].len();
    assert!(n <= m);
    let mut u = vec![0; n + 1];
    let mut v = vec![0; m + 1];
    let mut p = vec![0; m + 1];

    for i in 1..=n {
        let mut way = vec![0; m + 1];
        let mut used = vec![false; m + 1];
        let mut dist = vec![i64::MAX; m + 1];

        p[0] = i;
        let mut j0 = 0;
        loop {
            used[j0] = true;
            let i0 = p[j0];
            let mut j1 = 0;
            let mut delta = i64::MAX;

            for j in 1..=m {
                if !used[j] {
                    let current = cost[i0 - 1][j - 1] - u[i0] - v[j];
                    if current < dist[j] {
                        dist[j] = current;
                        way[j] = j0;
                    }
                    if dist[j] < delta {
                        delta = dist[j];
                        j1 = j;
                    }
                }
            }
            for j in 0..=m {
                if used[j] {
                    u[p[j]] += delta;
                    v[j] -= delta;
                } else {
                    dist[j] -= delta;
                }
            }
            j0 = j1;
            if p[j0] == 0 { break; }
        }
        loop {
            p[j0] = p[way[j0]];
            j0 = way[j0];
            if j0 == 0 { break; }
        }
    }
    -v[0]
}
