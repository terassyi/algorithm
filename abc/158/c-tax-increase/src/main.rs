fn main() {
    let (a, b) = input();
    println!("{}", solve(a, b));
}

fn input() -> (usize, usize) {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.pop();
    let mut iter = buf.split_whitespace();
    let a: usize = iter.next().unwrap().parse().unwrap();
    let b: usize = iter.next().unwrap().parse().unwrap();
    (a, b)
}

fn solve(a: usize, b: usize) -> i32 {
    for x in 1..1001 {
        let aa = (x * 8) / 100;
        let bb = x / 10;
        if aa == a && bb == b { return x as i32; }
    }
    -1
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {
        assert_eq!(super::solve(2, 2), 25);
    }
    #[test]
    fn test_solve2() {
        assert_eq!(super::solve(8,10),100);
    }
    #[test]
    fn test_solve3() {
        assert_eq!(super::solve(19,99), -1);
    }
}
