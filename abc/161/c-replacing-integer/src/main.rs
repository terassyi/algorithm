fn main() {
    let (n, k) = input();
    println!("{}", solve(n, k));
}

fn input() -> (u64, u64) {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.pop();
    let mut iter = buf.split_whitespace();
    let n: u64 = iter.next().unwrap().parse().unwrap();
    let k: u64 = iter.next().unwrap().parse().unwrap();
    (n, k)
}

fn solve(n: u64, k: u64) -> u64 {
    if n < k { return n; }
    if n % k == 0 { return 0; }
    let a = n % k;
    std::cmp::min(a, k-a)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {
        let n: u64 = 10000000000;
        let k: u64 = 1;
        assert_eq!(super::solve(n, k), 0);
    }
}
