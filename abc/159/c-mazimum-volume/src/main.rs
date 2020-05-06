fn main() {
    println!("{:?}", solve(input()));
}

fn input() -> f64 {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.pop();
    let l: f64 = buf.parse().unwrap();
    l
}

fn solve(l: f64) -> f64 {
    let a = l / 3.0;
    a * a * a
}
