fn main() {
    println!("Hello, world!");
}

fn solve(a: u64, b: u64) -> u64 {
    let mut is_prime: Vec<bool> = vec![true; (b-a) as usize];
    let mut is_prime_b_sq: Vec<bool> = Vec::new();
    let mut i: u64 = 0;
    while i*i <= b {
        is_prime_b_sq.push(true);
        i += 1;
    }
    is_prime_b_sq[0] = false;
    is_prime_b_sq[1] = false;

    for i in 0..is_prime_b_sq.len() {
        if is_prime_b_sq[i] {
            let mut j = i*2;
            while (j as usize) < is_prime_b_sq.len() {
                is_prime_b_sq[j as usize] = false;
                j += i;
            }
            let mut s: u64 = ((a+ (i as u64) -1) / (i as u64)) *i as u64;
            while s < b {
                is_prime[(s-a) as usize] = false;
                s += i as u64;
            }
        }
    }
    let mut counter = 0;
    for i in is_prime.iter() {
        if *i { counter += 1; }
    }
    counter
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve1() {
        assert_eq!(super::solve(22, 37), 3);
    }

    #[test]
    fn test_solve2() {
        assert_eq!(super::solve(22801753489, 22801787297), 1000);
    }
}
