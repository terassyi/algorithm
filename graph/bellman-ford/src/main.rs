fn main() {
    println!("Hello, world!");
}

struct Edge {
    from: usize,
    to: usize,
    cost: i32,
}

struct Graph {
    v: usize,
    e: usize,
    neighbors: Vec<Edge>,
}

impl Graph {
    fn bellman_ford(&self, start: usize) -> Vec<i32> {
        let inf: i32 = 1000000; // INF
        let mut d: Vec<i32> = vec![inf; self.v];
        // init start
        d[start] = 0;
        loop {
            let mut updated = false;
            for e in self.neighbors.iter() {
                if d[e.from] != inf && d[e.to] > d[e.from] + e.cost {
                    d[e.to] = d[e.from] + e.cost;
                    updated = true;
                }
            }
            if !updated { break; }
        }
        d
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_bellman_ford() {
        let graph = super::Graph {
            v: 7,
            e: 10,
            neighbors: vec![
                super::Edge { from: 0, to: 1, cost: 2},
                super::Edge { from: 1, to: 0, cost: 2},
                super::Edge { from: 0, to: 2, cost: 5},
                super::Edge { from: 2, to: 0, cost: 5},
                super::Edge { from: 1, to: 2, cost: 4},
                super::Edge { from: 2, to: 1, cost: 4},
                super::Edge { from: 1, to: 3, cost: 6},
                super::Edge { from: 3, to: 1, cost: 6},
                super::Edge { from: 1, to: 4, cost: 10},
                super::Edge { from: 4, to: 1, cost: 10},
                super::Edge { from: 2, to: 3, cost: 2},
                super::Edge { from: 3, to: 2, cost: 2},
                super::Edge { from: 3, to: 5, cost: 1},
                super::Edge { from: 5, to: 3, cost: 1},
                super::Edge { from: 4, to: 5, cost: 3},
                super::Edge { from: 5, to: 4, cost: 3},
                super::Edge { from: 4, to: 6, cost: 5},
                super::Edge { from: 6, to: 4, cost: 5},
                super::Edge { from: 5, to: 6, cost: 9},
                super::Edge { from: 6, to: 5, cost: 9},
            ],
        };
        let d = graph.bellman_ford(0);
        assert_eq!(d[6], 16);
    }
}
