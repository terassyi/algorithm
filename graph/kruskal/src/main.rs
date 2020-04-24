use std::cmp::Ordering;

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

impl Graph {
    fn kruskal(&self) -> usize {
        let mut edges: Vec<Edge> = Vec::new();
        for v in self.neighbors.iter() {
            for e in v.iter() {
                edges.push(*e);
            }
        }
        edges.sort(); // sort by cost
        // init union-find tree
        let mut union_find = UnionFind::new(self.v);
        let mut ans: usize = 0;

        for e in edges.iter() {
            if !union_find.is_same(e.from, e.to) {
                union_find.unite(e.from, e.to);
                ans += e.cost;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_kruskal() {
        let graph = super::Graph{
            v: 7,
            e: 9,
            neighbors: vec![
                vec![super::Edge{from:0 ,to: 2, cost: 1}],
                vec![super::Edge{from:1, to:2, cost:2}, super::Edge{from:1, to:4, cost:10}],
                vec![super::Edge{from:2, to:0, cost:1}, super::Edge{from:2, to:1, cost: 2}, super::Edge{from:2, to:3, cost: 3}, super::Edge{from:2, to:5, cost:7}],
                vec![super::Edge{from:3, to:2, cost:3}, super::Edge{from:3, to:5, cost:1}, super::Edge{from:3, to:6, cost:5}],
                vec![super::Edge{from:4, to:1, cost:10}, super::Edge{from:4, to:5, cost:5}],
                vec![super::Edge{from:5, to:2, cost:7}, super::Edge{from:5, to:3, cost:1}, super::Edge{from:5, to:6, cost:8}],
                vec![super::Edge{from:6, to:3, cost:5}, super::Edge{from:6, to:5, cost:8}]
            ],
        };
        let m = graph.kruskal();
        assert_eq!(m, 17);
    }
}
