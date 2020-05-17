use std::collections::BinaryHeap;

fn main() {
    println!("Hello, world!");
}

fn input() -> (usize, usize, Vec<(usize, usize)>) {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.pop();
    let mut iter = buf.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut lis: Vec<(usize, usize)> = Vec::new();
    for _ in 0..m {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let mut iter = buf.split_whitespace();
        let a: usize = iter.next().unwrap().parse().unwrap();
        let b: usize = iter.next().unwrap().parse().unwrap();
        lis.push((a-1, b-1));
    }
    (n, m, lis)
}

fn solve(n: usize, m: usize, lis: Vec<(usize, usize)>) -> (String, Option<Vec<usize>>) {
    
}