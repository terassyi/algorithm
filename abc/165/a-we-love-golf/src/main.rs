fn main() {
    let (k, a, b) = input();
    println!("{}", solve(k, a, b));
}

fn input() -> (u32, u32, u32) {
    let mut buf1 = String::new();
    let mut buf2 = String::new();
    std::io::stdin().read_line(&mut buf1).unwrap();
    std::io::stdin().read_line(&mut buf2).unwrap();
    buf1.pop();
    buf2.pop();
    let k: u32 = buf1.parse().unwrap();
    let mut iter = buf2.split_whitespace();
    let a: u32 = iter.next().unwrap().parse().unwrap();
    let b: u32 = iter.next().unwrap().parse().unwrap();
    (k, a, b)
}

fn solve(k: u32, a: u32, b: u32) -> String {
    if k == 1 { return String::from("OK"); }
    if a % k == 0 || b % k == 0 { return String::from("OK"); }
    if ((a/k)+1) * k <= b {
        return String::from("OK")
    }
    String::from("NG")
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve1() {
        assert_eq!(super::solve(7, 500, 600), String::from("OK"));
    }

    #[test]
    fn test_solve2() {
        assert_eq!(super::solve(4, 5, 7), String::from("NG"));
    }

    #[test]
    fn test_solve3() {
        assert_eq!(super::solve(1,11,11), String::from("OK"));
    }
}
