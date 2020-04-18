use std::cmp;

fn main() {
    println!("Hello, world!");
}

fn solve(n: usize, m: usize, s: &str, t: &str) -> u32 {
    let mut dp: Vec<Vec<u32>> = vec![vec![0u32; m+1]; n+1];

    for a in s.char_indices() {
        for b in t.char_indices(){
            if a.1 == b.1 {
                dp[a.0 + 1][b.0 + 1] = cmp::max(cmp::max(dp[a.0][b.0] + 1, dp[a.0][b.0 + 1]), dp[a.0 + 1][b.0]);
            } else {
                dp[a.0 + 1][b.0 + 1] = cmp::max(dp[a.0][b.0 + 1], dp[a.0 + 1][b.0]);
            }
        }
    }
    dp[n][m]
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {
        let n: usize = 4;
        let m: usize = 4;
        let s: String = String::from("abcd");
        let t: String = String::from("becd");
        assert_eq!(super::solve(n, m, &s, &t), 3);
    }
}
