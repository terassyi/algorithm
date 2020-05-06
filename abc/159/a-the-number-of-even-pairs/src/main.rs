fn main() {
    let (n, m) = input();
    println!("{}", solve(n, m));
}

fn input() -> (usize, usize) {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.pop();
    let mut iter = buf.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    (n, m)
}

fn solve(n: usize, m: usize) -> (usize) {
    // n! / r! * (n-r)!
    // n! / 2 * (n-2)! = n * (n-1) / 2
    (n * (n-1) / 2) + (m * (m-1) / 2)
}

