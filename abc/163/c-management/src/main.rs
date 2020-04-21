fn main() {
    let (n, a) = input();
    let ans = solve(n, a);
    for n in ans.iter() { println!("{}", n); }
}

fn input() -> (u64, Vec<u64>) {
    let mut buf1 = String::new();
    let mut buf2 = String::new();
    std::io::stdin().read_line(&mut buf1).unwrap();
    std::io::stdin().read_line(&mut buf2).unwrap();
    buf1.pop();
    buf2.pop();
    let n: u64 = buf1.parse().unwrap();
    let mut iter = buf2.split_whitespace();
    let a: Vec<u64> = iter.map(|x| x.parse().unwrap()).collect();
    (n, a)
}

fn solve(n: u64, a: Vec<u64>) -> Vec<u32> {
    let n: usize = n as usize;
    let mut subordinate: Vec<u32> = vec![0u32; n];
    for num in a.iter() {
        subordinate[*num as usize - 1] += 1;
    }
    subordinate
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve1() {
        let n: u64 = 5;
        let a: Vec<u64> = vec![1,1,2,2];
        assert_eq!(super::solve(n, a), vec![2,2,0,0,0]);
    }
    #[test]
    fn test_solve2() {
        let n: u64 = 10;
        let a: Vec<u64> = vec![1,1,1,1,1,1,1,1,1];
        assert_eq!(super::solve(n, a), vec![9,0,0,0,0,0,0,0,0,0]);
    }
    #[test]
    fn test_solve3() {
        let n: u64 = 7;
        let a: Vec<u64> = vec![1,2,3,4,5,6];
        assert_eq!(super::solve(n, a), vec![1,1,1,1,1,1,0]);
    }
}
