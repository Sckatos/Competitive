fn bfs(tree: &[Vec<u8>], start: u8) {
    let mut queue = std::collections::VecDeque::<u8>::new();
    let mut visited = vec![];
    queue.push_back(start);
    while !queue.is_empty() {
        let current = queue.pop_back().unwrap();
        if visited.contains(&current) { continue; }
        println!("{current}");
        visited.push(current);

        for &i in &tree[current as usize] {
            queue.push_front(i);
        }
    }
}

fn dfs(tree: &[Vec<u8>], start: u8) {
    let mut stack = vec![];
    let mut visited = vec![];

    stack.push(start);
    while !stack.is_empty() {
        let current = stack.pop().unwrap();
        if visited.contains(&current) { continue; }
        visited.push(current);
        println!("{current}");

        for &i in &tree[current as usize] {
            stack.push(i);
        }
    }
}
