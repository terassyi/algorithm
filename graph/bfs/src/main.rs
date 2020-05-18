use std::collections::VecDeque;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Edge {
    from: usize,
    to: usize,
    cost: i32,
}

#[derive(Debug, Clone, PartialEq)]
struct Graph {
    v: usize,
    e: usize,
    neighbors: Vec<Vec<Edge>>,
}

impl Graph {
    fn bfs(&self, start: usize) {
        let mut queue: VecDeque<Edge> = VecDeque::new();
        for e in self.neighbors[start].iter() {
            queue.push_back(*e);
        }
        while !queue.is_empty() {
            let e = queue.pop_front().unwrap();
            for t in self.neighbors[e.to].iter() {
                queue.push_back(*t);
            }
        }
    }
}
