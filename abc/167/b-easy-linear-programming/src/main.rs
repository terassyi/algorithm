fn main() {
    let (a, b, c, k) = input();
    println!("{}", solve(a, b, c, k));
}

fn input() -> (i32, i32, i32, i32) {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let a: i32 = iter.next().unwrap().parse().unwrap();
    let b: i32 = iter.next().unwrap().parse().unwrap();
    let c: i32 = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();
    (a, b, c, k)
}

fn solve(a: i32, b: i32, c: i32, k: i32) -> i32 {
    let mut res = 0;
    if a - k >= 0 {
        return k;
    } else {
        res = a;
    }
    let mut remain = k - a;
    if b - remain >= 0 {
        return res;
    }
    remain  = remain - b;
    res - remain
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {
        assert_eq!(super::solve(2,1,1,3), 2);
    }
    #[test]
    fn test_solve2() {
        assert_eq!(super::solve(1,2,3,4), 0);
    }
    #[test]
    fn test_solve3() {
        assert_eq!(super::solve(2000000000, 0, 0, 2000000000), 2000000000);
    }
}
