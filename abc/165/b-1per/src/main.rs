fn main() {
    println!("{:?}", solve(input()));
}

fn input() -> u64 {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.pop();
    let x: u64 = buf.parse().unwrap();
    x
}

fn solve(x: u64) -> u64 {
    let mut amount: u64 = 100;
    let mut counter = 0;
    loop {
        amount += amount / 100;
        counter += 1;
        if x <= amount { break; }
        
    }
    counter
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve1() {
        assert_eq!(super::solve(103), 3);
    }
    #[test]
    fn test_solve2() {
        assert_eq!(super::solve(1000000000000000000), 3760);
    }
    #[test]
    fn test_solve3() {
        assert_eq!(super::solve(1333333333), 1706);
    }
}
