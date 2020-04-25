use std::cmp::Ordering;
use std::collections::BinaryHeap;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone, Copy)]
struct Edge {
    from: usize,
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

impl Edge {
    fn new(from: usize, to: usize, cost: usize) -> Edge {
        Edge{
            from: from,
            to: to,
            cost: cost,
        }
    }
}

impl Graph {
    fn new(n: usize, r: usize, lis: Vec<Vec<(usize, usize)>>) -> Self {
        Graph {
            v: n,
            e: r,
            neighbors: lis.iter().enumerate().map(|(from, v)| v.iter().map(|(to, cost)| Edge::new(from, *to, *cost)).collect()).collect()
        }
    }

    fn solve(&self, start: usize) -> usize {
        let inf = 1000000;
        let mut d1 = vec![inf; self.v];
        let mut d2 = vec![inf; self.v];
        let mut pair: BinaryHeap<(usize, usize)> = BinaryHeap::new();
        pair.push((0,start));
        d1[start] = 0;
        d2[start] = 0;

        loop {
            if pair.is_empty() { break; }
            let (cost, v) = pair.pop().unwrap();
            if d2[v] < cost { continue; }
            for e in self.neighbors[v].iter() {
                let new_cost = cost + e.cost;
                if d1[e.to] > new_cost {
                    d1[e.to] = new_cost;
                    pair.push((new_cost, e.to));
                }
                if d2[e.to] > new_cost && d1[e.to] < new_cost{
                    d2[e.to] = new_cost;
                    pair.push((new_cost, e.to));
                }
            }
            

        }
        d2[self.v-1]
    }
}

fn solve(n: usize, r: usize, lis: Vec<Vec<(usize, usize)>>) -> usize {
    let graph = Graph::new(n, r, lis);
    graph.solve(0)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {
        let n: usize = 4;
        let r: usize = 4;
        let lis: Vec<Vec<(usize, usize)>> = vec![
            vec![(1, 100)],
            vec![(0, 100), (2, 250), (3, 200)],
            vec![(1, 250), (3, 100)],
            vec![(1, 200), (2, 100)],
        ];
        assert_eq!(super::solve(n, r, lis), 450);
    }
}
