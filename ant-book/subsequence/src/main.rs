fn main() {
    println!("Hello, world!");
}

// しゃくとり法
fn solve(n: usize, s: usize, a: Vec<usize>) -> usize {
    let mut res = n+1;
    let mut b = 0;
    let mut t = 0;
    let mut sum = 0;
    loop {
        while t < n && sum < s {
            sum += a[t];
            t += 1;
        }
        if sum < s {
            break;
        }
        res = std::cmp::min(res, t - b);
        sum -= a[b];
        b += 1;
    }
    if res > n { return 0; }
    res
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {
        let a: Vec<usize> = vec![5,1,3,5,10,7,4,9,2,8];
        assert_eq!(super::solve(10,15,a), 2);
    }
}
