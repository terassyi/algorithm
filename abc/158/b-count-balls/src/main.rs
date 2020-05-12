fn main() {
    let (n, a, b) = input();
    println!("{}", solve(n, a, b));
}

fn input() -> (usize, usize, usize) {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.pop();
    let mut iter = buf.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let a: usize = iter.next().unwrap().parse().unwrap();
    let b: usize = iter.next().unwrap().parse().unwrap();
    (n, a, b)
}

fn solve(n: usize, a: usize, b: usize) -> usize {
    let c = (n / (a+b));
    let d = n - c * (a + b);
    if d < a {
        return a * c + d
    }
    a * (c + 1)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {
        assert_eq!(super::solve(8,3,4), 4);
    }
    #[test]
    fn test_solve2() {
        assert_eq!(super::solve(8,0,4),0);
    }
    #[test]
    fn test_solve3() {
        assert_eq!(super::solve(6,2,4),2);
    }
}
