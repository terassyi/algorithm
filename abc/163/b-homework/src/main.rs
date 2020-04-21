fn main() {
    let (n, m, a) = input();
    println!("{}", solve(n, m, a));
}

fn input() -> (u64, u64, Vec<u64>) {
    let mut buf1 = String::new();
    let mut buf2 = String::new();
    std::io::stdin().read_line(&mut buf1).unwrap();
    std::io::stdin().read_line(&mut buf2).unwrap();
    buf1.pop();
    buf2.pop();
    let mut buf1 = buf1.split_whitespace();
    let n: u64 = buf1.next().unwrap().parse().unwrap();
    let m: u64 = buf1.next().unwrap().parse().unwrap();
    let mut iter = buf2.split_whitespace();
    let a: Vec<u64> = iter.map(|x| x.parse().unwrap()).collect();
    (n, m, a)
}

fn solve(n: u64, m: u64, a: Vec<u64>) -> i64 {
    let sum = a.iter().sum();
    if n < sum { return -1; }
    (n - sum) as i64
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve1() {
        let n: u64 = 41;
        let m: u64 = 2;
        let a: Vec<u64> = vec![5, 6];
        assert_eq!(super::solve(n, m, a), 30);
    }
    #[test]
    fn test_solve2() {
        let n: u64 = 10;
        let m: u64 = 2;
        let a: Vec<u64> = vec![5, 6];
        assert_eq!(super::solve(n, m, a), -1);
    }
    #[test]
    fn test_solve3() {
        let n: u64 = 314;
        let m: u64 = 15;
        let a: Vec<u64> = vec![9,26,5,35,8,9,79,3,23,8,46,2,6,43,3];
        assert_eq!(super::solve(n, m, a), 9);
    }
}
