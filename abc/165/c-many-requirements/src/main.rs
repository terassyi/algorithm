use itertools::Itertools;

fn main() {
    let (n, m, q, v) = input();
    println!("{}", solve(n, m, q, v));
}

fn input() -> (usize, usize, usize, Vec<(usize, usize, usize, usize)>) {
    let mut buf1 = String::new();
    let mut buf2 = String::new();
    
    std::io::stdin().read_line(&mut buf1).unwrap();
    std::io::stdin().read_line(&mut buf2).unwrap();
    buf1.pop();
    buf2.pop();
    let mut iter1 = buf1.split_whitespace();
    let mut iter2 = buf2.split_whitespace();

    let n: usize = iter1.next().unwrap().parse().unwrap();
    let m: usize = iter1.next().unwrap().parse().unwrap();
    let q: usize = iter2.next().unwrap().parse().unwrap();

    let mut v: Vec<(usize, usize, usize, usize)> = Vec::new();
    for _ in 0..q {
        let mut buf3 = String::new();
        std::io::stdin().read_line(&mut buf3).unwrap();
        buf3.pop();
        let mut iter3 = buf3.split_whitespace();
        let a: usize = iter3.next().unwrap().parse().unwrap();
        let b: usize = iter3.next().unwrap().parse().unwrap();
        let c: usize = iter3.next().unwrap().parse().unwrap();
        let d: usize = iter3.next().unwrap().parse().unwrap();
        v.push((a, b, c, d));
    }
    (n, m, q, v)
}

fn solve(n: usize, m: usize, q: usize, v: Vec<(usize, usize, usize, usize)>) -> usize {
    let t = (1..m+1).combinations_with_replacement(n)
            .map(|A| 
                v.iter()
                .filter(|(a, b, c, _d)| A[*b-1] - A[*a-1] == *c )
                .fold(0, |sum, (_, _, _, d)| sum + d )
            );
    t.max().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {
        /*
        3 4 3
1 3 3 100
1 2 2 10
2 3 2 10
        */
        let n: usize = 3;
        let m: usize = 4;
        let q: usize = 3;
        let v: Vec<(usize, usize, usize, usize)> = vec![(1,3,3,100), (1,2,2,10), (2,3,2,10)];
        assert_eq!(super::solve(n, m, q, v), 110);
    }
    #[test]
    fn test_solve2() {
        let n: usize = 4;
        let m: usize = 6;
        let q: usize = 10;
        let v: Vec<(usize, usize, usize, usize)> = vec![
            (2, 4, 1, 86568),
            (1, 4, 0, 90629),
            (2, 3, 0, 90310),
            (3, 4, 1, 29211),
            (3, 4, 3, 78537),
            (3, 4, 2, 8580),
            (1, 2, 1, 96263),
            (1, 4, 2, 2156),
            (1, 2, 0, 94325),
            (1, 4, 3, 94328),
        ];
        assert_eq!(super::solve(n, m, q, v), 357500);
    }
}
