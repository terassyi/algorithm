fn main() {
    println!("Hello, world!");
}

fn solve(n: u64) -> u64 {
    let mut is_prime: Vec<bool> = vec![true; n as usize + 1];
    let mut p = 0;
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..n+1 {
        if is_prime[i as usize] {
            let mut j = i*2;
            while j <= n {
                is_prime[j as usize] = false;
                j += i;
            }
        }
    }
    let mut counter = 0;
    for b in is_prime.iter() {
        if *b { counter += 1; }
    }
    counter
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {
        assert_eq!(super::solve(1000000), 78498);
    }
}
