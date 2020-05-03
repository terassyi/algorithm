fn main() {
    let (a, b, n) = input();
    println!("{}", solve(a, b, n));
}

fn input() -> (usize, usize, usize) {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.pop();
    let mut iter = buf.split_whitespace();
    let a: usize = iter.next().unwrap().parse().unwrap();
    let b: usize = iter.next().unwrap().parse().unwrap();
    let n: usize = iter.next().unwrap().parse().unwrap();
    (a, b, n)
}

fn solve(a: usize, b: usize, n: usize) -> usize {
    if b-1 < n {
        return (a*(b-1))/b - a*((b-1)/b);
    }
    (a*n)/b - a*(n/b)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve1() {
        assert_eq!(super::solve(5, 7, 4), 2);
    }
    #[test]
    fn test_solve2() {
        assert_eq!(super::solve(11, 10, 9), 9);
    }
}
