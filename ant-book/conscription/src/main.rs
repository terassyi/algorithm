use std::cmp::Ordering;
use std::collections::BinaryHeap;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone, Copy)]
struct Edge {
    from: usize,
    to: usize,
    cost: i32,
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

#[derive(Debug, PartialEq, Eq, Clone)]
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        UnionFind {
            parent: (0..size).collect(),
            rank: vec![0usize; size],
        }
    }

    fn find(&mut self, val: usize) -> usize {
        if self.parent[val] == val {
            return val; 
        }
        self.parent[val] = self.find(self.parent[val]);
        self.parent[val]
    }

    fn unite(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);
        if x == y { return; }
        if self.rank[x] < self.rank[y] {
            self.parent[x] = y;
        } else {
            self.parent[y] = x;
            if self.rank[x] == self.rank[y] {
                self.rank[x] += 1;
            }
        }
    }

    fn is_same(&mut self, x: usize, y: usize) -> bool {
        let a = self.find(x);
        let b = self.find(y);
        a == b
    }
}

impl Edge {
    fn new(from: usize, to: usize, cost: i32) -> Edge {
        Edge{
            from: from,
            to: to,
            cost: cost,
        }
    }
}

impl Graph {
    fn new(n: usize, m: usize, r: usize, lis: Vec<(usize, usize, usize)>) -> Self {
        Graph {
            v: n+m,
            e: r,
            neighbors: make_neighbors(n, m, lis),
        }
    }

    fn solve(&mut self) -> i32 {
        // minimum spanning tree
        let res = self.kruskal();
        10000 * self.v as i32 + res
    }

    fn kruskal(&self) -> i32 {
        let mut edges: Vec<Edge> = Vec::new();
        for v in self.neighbors.iter() {
            for e in v.iter() {
                edges.push(*e);
            }
        }
        edges.sort(); // sort by cost
        // init union-find tree
        let mut union_find = UnionFind::new(self.v);
        let mut ans: i32 = 0;

        for e in edges.iter() {
            if !union_find.is_same(e.from, e.to) {
                union_find.unite(e.from, e.to);
                ans += e.cost;
            }
        }
        ans
    }
}

fn make_neighbors(n: usize, m: usize, lis: Vec<(usize, usize, usize)>) -> Vec<Vec<Edge>> {
    let mut neighbors: Vec<Vec<Edge>> = vec![Vec::new(); n+m];
    for (x, y, d) in lis.iter() {
        let edge = Edge::new(*x, y+n, -(*d as i32));
        let rev_edge = Edge::new(y+n, *x, -(*d as i32));
        neighbors[*x].push(edge);
        neighbors[y+n].push(rev_edge);
    }
    neighbors
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {
        let n: usize = 5;
        let m: usize = 5;
        let r: usize = 8;
        let lis: Vec<(usize, usize, usize)> = vec![
            (4,3,6831), (1,3,4583), (0,0,6592), (0,1,3063), (3,3,4975), (1,3,2049), (4,2,2104), (2,2,781),
        ];
        let mut graph = super::Graph::new(n, m, r, lis);
        let ans = graph.solve();
        assert_eq!(ans, 71071);
    }
}
