fn main() {
    let (n, k, v) = input();
    println!("{}", solve(n, k, v));
}

fn input() -> (usize, usize, Vec<Vec<usize>>) {
    let mut buf1 = String::new();
    std::io::stdin().read_line(&mut buf1).unwrap();
    buf1.pop();
    let mut iter1 = buf1.split_whitespace();
    let n: usize = iter1.next().unwrap().parse().unwrap();
    let k: usize = iter1.next().unwrap().parse().unwrap();
    let mut v = Vec::new();
    for i in 0..k {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        buf.pop();
        let d: usize = buf.parse().unwrap();
        let mut buf2 = String::new();
        std::io::stdin().read_line(&mut buf2).unwrap();
        buf2.pop();
        let mut iter2 = buf2.split_whitespace();
        let vv: Vec<usize> = iter2.map(|s| s.parse().unwrap() ).collect();
        v.push(vv);
    }
    (n, k, v)
}

fn solve(n: usize, k: usize, v: Vec<Vec<usize>>) -> usize {
    let mut have: Vec<bool> = vec![false; n];
    for i in v.iter() {
        for a in i.iter() {
            if !have[a-1] { have[a-1] = true; }
        }
    }
    let mut ans = 0;
    for h in have.iter() {
        if !h { ans += 1; }
    }
    ans
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {
        let v: Vec<Vec<usize>> = vec![
            vec![1,3],
            vec![3],
        ];
        assert_eq!(super::solve(3, 2, v), 1);
    }
    #[test]
    fn test_solve2() {
        let v: Vec<Vec<usize>> = vec![
            vec![3],
            vec![3],
            vec![3],
        ];
        assert_eq!(super::solve(3,3,v), 2);
    }
}
