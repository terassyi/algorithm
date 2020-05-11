use itertools::Itertools;

fn main() {
    let (n, m, x, mut c) = input();
    println!("{}", solve(n, m, x, &mut c))
}

fn input() -> (usize, usize, usize, Vec<Vec<usize>>) {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.pop();
    let mut iter = buf.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let x: usize = iter.next().unwrap().parse().unwrap();

    let mut c: Vec<Vec<usize>> = Vec::new();
    for _ in 0..n {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        buf.pop();
        let iter = buf.split_whitespace();
        c.push(iter.map(|v| v.parse().unwrap()).collect());
    }
    (n, m, x, c)
}

fn solve(n:usize, m:usize, x:usize, c: &mut Vec<Vec<usize>>) -> i64 {
   let mut ans = 100000000;
    for i in 1..n+1 {
        for v in (0..n+1).combinations(i) {
            if is_over_x(x, m, &v, c) {
                ans = std::cmp::min(ans, sum(&v, c));
            }
        }
    }
    if ans == 100000000 { return -1; }
    ans as i64
}

fn sum(buy: &Vec<usize>, c: &Vec<Vec<usize>>) -> usize {
    let mut res = 0;
    for i in buy.iter() {
        res += c[*i][0];
    }
    res
}

fn is_over_x(x: usize, m: usize, buy: &Vec<usize>, c: &Vec<Vec<usize>>) -> bool {
    let mut sum = vec![0usize; m];
    for i in buy.iter() {
        for j in 1..m+1 {
            sum[j-1] += c[*i][j]
        }
    }
    for s in sum.iter() {
        if *s < x { return false; }
    }
    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {
        let n: usize = 3;
        let m: usize = 3;
        let x: usize = 10;
        let mut c: Vec<Vec<usize>> = vec![
            vec![60, 2,2,4],
            vec![70,8,7,9],
            vec![50,2,3,9],
        ];

        assert_eq!(super::solve(n, m, x, &mut c), 120);
    }

    #[test]
    fn test_is_over_x() {
        let n: usize = 3;
        let m: usize = 3;
        let x: usize = 10;
        let mut c: Vec<Vec<usize>> = vec![
            vec![60, 2,2,4],
            vec![70,8,7,9],
            vec![50,2,3,9],
        ]; 
        assert_eq!(super::is_over_x(x, m, &vec![1,2], &c), true);
    }
}
