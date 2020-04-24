use std::collections::BinaryHeap;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone)]
struct Edge {
    to: usize,
    cost: i32,
}

#[derive(Debug, Clone)]
struct Graph {
    v: usize,
    e: usize,
    neighbors: Vec<Vec<Edge>>,
}

impl Graph {
    fn dijkstra(&self, start: usize) -> Vec<usize> {
        // init
        let inf = 1000000;
        let mut d = vec![inf; self.v];
        let mut pair: BinaryHeap<(usize, usize)> = BinaryHeap::new();
        pair.push((0, start));
        d[start] = 0;

        loop {
            if pair.is_empty() { break; }
            let (path, edge) = pair.pop().unwrap();
            if d[edge] < path { continue; }
            println!("neighbir list: {:?}", self.neighbors[edge]);
            for e in self.neighbors[edge].iter() {
                if d[e.to] > d[edge]+e.cost as usize {
                    d[e.to] = d[edge]+e.cost as usize;
                    pair.push((d[e.to], e.to));
                }
            }
        }
        d
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_dijkstra() {
        let start: usize = 0;
        let graph = super::Graph {
            v: 7,
            e: 10,
            neighbors: vec![
                vec![super::Edge{to: 1, cost: 2}, super::Edge{to: 2, cost: 5}],
                vec![super::Edge{to: 0, cost: 2}, super::Edge{to: 2, cost: 4}, 
                    super::Edge{to: 3, cost: 6}, super::Edge{to: 4, cost: 10}],
                vec![super::Edge{to: 0, cost: 5}, super::Edge{to: 1, cost: 4}, super::Edge{to: 3, cost: 2}],
                vec![super::Edge{to: 2, cost: 2}, super::Edge{to: 1, cost: 6}, super::Edge{to: 5, cost: 1}],
                vec![super::Edge{to: 1, cost: 10}, super::Edge{to: 5, cost: 3}, super::Edge{to: 6, cost: 5}],
                vec![super::Edge{to: 3, cost: 1}, super::Edge{to: 4, cost: 3}, super::Edge{to: 6, cost: 9}],
                vec![super::Edge{to: 4, cost: 5}, super::Edge{to: 5, cost: 9}],
            ],
        };

        let d = graph.dijkstra(start);
        assert_eq!(d[6], 16);
    }
}
