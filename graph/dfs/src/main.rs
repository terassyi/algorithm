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
    fn dfs(&self, start: usize, end: usize) {
        let mut stack: Vec<Edge> = Vec::new();
        for e in self.neighbors[start].iter() {
            stack.push(*e);
        }
        while !stack.is_empty() {
            let e = stack.pop().unwrap();
            for t in self.neighbors[e.to].iter() {
                stack.push(*t);
            }
            if e.to == end {
                
            }
        }
    }
}
