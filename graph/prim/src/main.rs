use std::collections::BinaryHeap;
use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone, Copy)]
struct Edge {
    to: usize,
    cost: usize,
}

#[derive(Debug, Clone)]
struct Graph {
    v: usize,
    e: usize,
    neighbors: Vec<Vec<Edge>>,
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Edge) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Edge) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Edge) -> bool {
        self.cost == other.cost
    }
}

impl Eq for Edge {}

impl Graph {
    fn prim(&mut self) -> usize {
        let inf: usize = 1000000;
        let mut mincost = vec![inf; self.v];
        let mut used = vec![false; self.v];
        let mut queue: BinaryHeap<Edge> = BinaryHeap::new();
        let mut res: usize = 0;
        // mincost[0] = 0;
        let start = self.neighbors[0].iter().min().unwrap();
        queue.push(*start);
        used[0] = true;
        res += start.cost;
        loop {
            if queue.is_empty() { break; }
            let vertex = queue.pop().unwrap();
            self.neighbors[vertex.to].sort_by(|v1, v2| v1.cost.cmp(&v2.cost));
            // println!("{:?}", self.neighbors[vertex]);
            for v in self.neighbors[vertex.to].iter() {
                if !used[v.to] {
                    res += v.cost;
                    used[v.to] = true;
                    queue.push(*v);
                    break;
                }
            }

        }
        res
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_prim() {
        let mut graph = super::Graph{
            v: 7,
            e: 9,
            neighbors: vec![
                vec![super::Edge{to: 2, cost: 1}],
                vec![super::Edge{to:2, cost:2}, super::Edge{to:4, cost:10}],
                vec![super::Edge{to:0, cost:1}, super::Edge{to:1, cost: 2}, super::Edge{to:3, cost: 3}, super::Edge{to:5, cost:7}],
                vec![super::Edge{to:2, cost:3}, super::Edge{to:5, cost:1}, super::Edge{to:6, cost:5}],
                vec![super::Edge{to:1, cost:10}, super::Edge{to:5, cost:5}],
                vec![super::Edge{to:2, cost:7}, super::Edge{to:3, cost:1}, super::Edge{to:6, cost:8}],
                vec![super::Edge{to:3, cost:5}, super::Edge{to:5, cost:8}]
            ],
        };
        let m = graph.prim();
        assert_eq!(m, 17);
    }
}
