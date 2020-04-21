fn main() {
    let (n, a) = input();
    println!("{}", solve(n, a));
}

fn input() -> (i64, Vec<i64>) {
    let mut buf1 = String::new();
    let mut buf2 = String::new();
    buf1.pop();
    buf2.pop();
    let n: i64 = buf1.parse().unwrap();
    let buf = buf2.split_whitespace();
    let a: Vec<i64> = buf.map(|b| b.parse().unwrap()).collect();
    (n, a)
}

fn solve(n: i64, a: Vec<i64>) -> i64 {
    let n: usize = n as usize;
    let mut pair = Vec::new();
    for (i, v) in a.iter().enumerate() {
        pair.push((v, i));
    }
    pair.sort();
    pair.reverse();
    let mut dp = vec![vec![0i64; (n+1) as uszie]; (n+1) as uszie];
    for x in 0..n {
        let (val, idx) = pair[x];
        for y in 0..n {
            if x+y >= n { break; }
            let n = n as i64;
            dp[x+1][y] = std::cmp::max(dp[x+1][y], dp[x][y] + val * (x-idx).abs());
            dp[x][y+1] = std::cmp::max(dp[x][y+1], dp[x][y] + val * (n-idx-(y as i64)-1).abs());
        }
    }
    let mut ans: i64 = 0;
    for i in 0..n {
        ans = std::cmp::max(ans, dp[i as usize][(n-i) as usize]);
    }
    ans
}
