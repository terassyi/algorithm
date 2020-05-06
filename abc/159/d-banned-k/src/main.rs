fn main() {
    let (n, a) = input();
    let ans = solve(n, a);
    for v in ans.iter() { println!("{}", v); }
}

fn input() -> (usize, Vec<usize>) {
    let mut buf1 = String::new();
    let mut buf2 = String::new();
    std::io::stdin().read_line(&mut buf1).unwrap();
    std::io::stdin().read_line(&mut buf2).unwrap();
    buf1.pop();
    buf2.pop();
    let n: usize = buf1.parse().unwrap();
    let iter = buf2.split_whitespace();
    let a: Vec<usize> = iter.map(|s| s.parse().unwrap()).collect();
    (n, a)
}

fn solve(n: usize, a: Vec<usize>) -> Vec<usize> {
    let mut counter = vec![0usize; n+1];
    let mut pairs = vec![0usize; n+1];
    for v in a.iter() {
        counter[v-1] += 1;
    }
    for (i, c) in counter.iter().enumerate() {
        pairs[i] = get_pair(*c);
    }
    let sum: usize = pairs.iter().sum();
    let mut ans: Vec<usize> = Vec::new();
    for k in a.iter() {
        ans.push(sum - (counter[k-1] - 1));
    }
    ans
}

fn get_pair(n: usize) -> usize {
    if n == 0 { return 0; }
    (n * (n-1)) / 2
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {
        assert_eq!(super::solve(5, vec![1,1,2,1,2]), vec![2,2,3,2,3]);
    }
    #[test]
    fn test_solve2() {
        assert_eq!(super::solve(5, vec![3,3,3,3,3]), vec![6,6,6,6,6]);
    }
}
