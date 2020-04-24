fn main() {
    let n: usize = 100;
    let k: usize = 7;
    let info: Vec<(usize, usize, usize)> = vec![(1,101,1),(2,1,2),(2,2,3),(2,3,3),(1,1,3),(2,3,1),(1,5,5)];

    let ans = solve(n, k, info);
    println!("{}", ans);
}

#[derive(Debug, PartialEq, Eq)]
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
        if self.parent[val] == val { return val; }
        else {
            self.parent[val] = self.find(val);
            return self.parent[val];
        }
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

#[warn(unused_variables)]
fn solve(n: usize, k: usize, info: Vec<(usize, usize, usize)>) -> usize {
    // init union-find tree
    let mut tree: UnionFind = UnionFind::new(n * 3);
    let mut ans: usize = 0;

    for (typ, x, y) in info.iter() {
        let x = x -1;
        let y = y -1;
        if n <= x || n <= y {
            ans += 1;
            continue;
        }
        if *typ == 1 {
            // x and y is same type
            if tree.is_same(x, y+n) || tree.is_same(x, y+2*n) {
                ans += 1;
            } else {
                tree.unite(x, y);
                tree.unite(x+n, y+n);
                tree.unite(x+2*n, y+2*n);
            }
        } else {
            // x eats y
            if tree.is_same(x, y) || tree.is_same(x, y+2*n) {
                ans += 1;
            } else {
                tree.unite(x, y+n);
                tree.unite(x+n, y+2*n);
                tree.unite(x+2*n, y);
            }
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {
        let n: usize = 100;
        let k: usize = 7;
        let info: Vec<(usize, usize, usize)> = vec![(1,101,1),(2,1,2),(2,2,3),(2,3,3),(1,1,3),(2,3,1),(1,5,5)];
        assert_eq!(super::solve(n, k, info), 3);
    }
}
