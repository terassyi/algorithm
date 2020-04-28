fn main() {
    println!("Hello, world!");
}

fn mod_pow(x: u64, n: u64, m: u64) -> u64 {
    let mut x = x;
    let mut n = n;
    let mut res = 1;
    while n > 0 {
        if n%2 == 1 {
            res = res * x % m;
        }
        x = x * x % m;
        n = n / 2;
    }
    res
}

fn is_prime(n: u64) -> bool {
    let mut i: u64 = 2;
    while i*i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

fn solve(n: u64) -> bool {
    if is_prime(n) { return false; }
    for x in 2..n {
        let result = mod_pow(x, n, n);
        if result == x { return true; }
    }
    false
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve1() {
        assert_eq!(super::solve(17), false);
    }
    #[test]
    fn test_solve2() {
        assert_eq!(super::solve(561), true);
    }
    #[test]
    fn test_solve3() {
        assert_eq!(super::solve(4), false);
    }
}
