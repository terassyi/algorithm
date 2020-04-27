fn main() {
    println!("Hello, world!");
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

fn devisor(n: u64) -> Vec<u64> {
    let mut result: Vec<u64> = Vec::new();
    let mut i: u64 = 1;
    while i*i <= n {
        if n % i == 0 {
            result.push(i);
            if i != n/i {
                result.push(n/i);
            }
        }
        i += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_is_prime() {
        assert_eq!(super::is_prime(53), true);
    }

    #[test]
    fn test_devisor() {
        assert_eq!(super::devisor(295927), vec![1, 295927, 541, 547]);
    }
}
