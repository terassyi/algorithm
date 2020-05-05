fn main() {
    let (_, _, h, path) = input();
    println!("{}", solve(h, path));
}

struct Hill {
    height: usize,
    path: Vec<usize>,
}

impl Hill {
    fn new(height: usize, path: Vec<usize>) -> Self {
        Hill {
            height: height,
            path: path,
        }
    }
    fn push_path(&mut self, path: usize) {
        self.path.push(path);
    }
}

fn input() -> (usize, usize, Vec<usize>, Vec<(usize, usize)>) {
    let mut buf1 = String::new();
    std::io::stdin().read_line(&mut buf1).unwrap();
    buf1.pop();
    let mut iter1 = buf1.split_whitespace();
    let n: usize = iter1.next().unwrap().parse().unwrap();
    let m: usize = iter1.next().unwrap().parse().unwrap();
    let mut buf2 = String::new();
    std::io::stdin().read_line(&mut buf2).unwrap();
    buf2.pop();
    let iter2 = buf2.split_whitespace();
    let h: Vec<usize> = iter2.map(|i| i.parse().unwrap() ).collect();
    let mut path: Vec<(usize, usize)> = Vec::new();
    for i in 0..m {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        buf.pop();
        let mut iter = buf.split_whitespace();
        let a: usize = iter.next().unwrap().parse().unwrap();
        let b: usize = iter.next().unwrap().parse().unwrap();
        path.push((a, b));
    }
    (n, m, h, path)
}

fn from_input(h: Vec<usize>, path: Vec<(usize, usize)>) -> Vec<Hill> {
    // make empty path hills
    let mut hills: Vec<Hill> = h.iter().map(|v| Hill{height: *v, path: vec![]}).collect();
    for p in path.iter() {
        hills[p.0 - 1].push_path(p.1-1);
        hills[p.1-1].push_path(p.0-1);
    }
    hills
}

fn solve(h: Vec<usize>, path: Vec<(usize, usize)>) -> usize {
    let hills = from_input(h, path);
    let mut ans = 0;
    for hill in hills.iter() {
        let mut high = true;
        for p in hill.path.iter() {
            if hills[*p].height >= hill.height {
                high = false;
                break;
            }
        }
        if high {
            ans += 1;
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {
        let h: Vec<usize> = vec![1,2,3,4];
        let path: Vec<(usize, usize)> = vec![(1,3),(2,3),(2,4)];
        assert_eq!(super::solve(h, path), 2);
    }
    #[test]
    fn test_solve2() {
        let h: Vec<usize> = vec![8,6,9,1,2,1];
        let path: Vec<(usize, usize)> = vec![(1,3),(4,2),(4,3),(4,6),(4,6)];
        assert_eq!(super::solve(h, path),3);
    }
}
