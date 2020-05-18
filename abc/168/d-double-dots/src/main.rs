use std::collections::VecDeque;

fn main() {
    let (n, m, lis) = input();
    let (ans, ll) = solve(n, m, lis);
    if let Some(l) = ll {
        println!("{}", ans);
        for p in l.iter() {
            println!("{}", p)
        }
    }
    println!("{:?}", ans);
}

fn input() -> (usize, usize, Vec<(usize, usize)>) {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.pop();
    let mut iter = buf.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut lis: Vec<(usize, usize)> = Vec::new();
    for _ in 0..m {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let mut iter = buf.split_whitespace();
        let a: usize = iter.next().unwrap().parse().unwrap();
        let b: usize = iter.next().unwrap().parse().unwrap();
        lis.push((a-1, b-1));
    }
    (n, m, lis)
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Edge {
    from: usize,
    to: usize,
}

#[derive(Debug, Clone, PartialEq)]
struct Graph {
    v: usize,
    e: usize,
    neighbors: Vec<Vec<Edge>>,
}

impl Graph {
    fn new(n: usize, m: usize, lis: Vec<(usize, usize)>) -> Self {
        let mut neighbors: Vec<Vec<Edge>> = vec![Vec::new(); m];
        for (from, to) in lis.iter() {
            let edge1 = Edge { 
                from: *from,
                to: *to,
            };
            let edge2 = Edge {
                from: *to,
                to: *from,
            };
            neighbors[*from].push(edge1);
            neighbors[*to].push(edge2);
        }
        Graph { 
            v: n,
            e: m,
            neighbors: neighbors,
        }
    }

    fn solve(&self, start: usize) -> Vec<usize> {
        let mut path: Vec<usize> = vec![10000; self.v];
        let mut queue: VecDeque<Edge> = VecDeque::new();
        for e in self.neighbors[start].iter() {
            queue.push_back(*e);
        }
        while !queue.is_empty() {
            let e = queue.pop_front().unwrap();
            for t in self.neighbors[e.to].iter() {
                if path[t.from] == 10000 {
                    path[t.from] = e.from;
                    queue.push_back(*t);
                }
                
            }
        }
        println!("{:?}", path);
        path
    }
}

fn solve(n: usize, m: usize, lis: Vec<(usize, usize)>) -> (String, Option<Vec<usize>>) {
    let graph = Graph::new(n, m, lis);
    println!("{:?}", graph);
    let path = graph.solve(0);
    for p in path.iter() {
        if *p == 10000 {
            return (String::from("No"), None);
        }
    }
    (String::from("Yes"), Some(path[1..].to_vec()))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {
        let lis = vec![
        (0, 1),
        (1, 2),
        (2, 3),
        (3, 1),
        ];
        assert_eq!(super::solve(4, 4, lis), (String::from("Yes"), Some(vec![1,2,2])))
    }
}
