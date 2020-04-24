fn main() {
    println!("Hello, world!");
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_new() {
        let a = super::UnionFind::new(5);
        let wanted = super::UnionFind{
            parent: vec![0,1,2,3,4],
            rank: vec![0,0,0,0,0],
        };
        assert_eq!(a, wanted);
    }

    #[test]
    fn test_find() {
        let mut a = super::UnionFind{
            parent: vec![0,1,2,0,4],
            rank: vec![0,0,0,1,0]
        };
        let i = a.find(3);
        assert_eq!(i, 0);
    }

    #[test]
    fn test_unite() {
        let mut a = super::UnionFind{
            parent: vec![0,1,2,3,4],
            rank: vec![0,0,0,0,0],
        };
        let wanted = super::UnionFind{
            parent: vec![0,1,2,0,4],
            rank: vec![1,0,0,0,0],
        };
        a.unite(0,3);
        assert_eq!(a, wanted);
    }
}
