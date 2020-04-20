fn main() {
    println!("{}", solve(input()));
}

fn input() -> u64 {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.pop();

    let n: u64 = buf.parse().unwrap();
    n
}

fn solve(n: u64) -> u64 {
    let all_sum = n * (n + 1) / 2;
    let sum_3 = ((n/3) * ((n/3)+1) / 2) * 3;
    let sum_5 = ((n/5) * ((n/5)+1) / 2) * 5;
    let sum_15 = ((n/15) * ((n/15)+1) / 2) * 15;

    all_sum - (sum_3 + sum_5 - sum_15)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve1() {
        let n: u64 = 15;
        assert_eq!(super::solve(n), 60);
    }

    #[test]
    fn test_solve2() {
        let n: u64 = 1000000;
        assert_eq!(super::solve(n), 266666333332);
    }
}
