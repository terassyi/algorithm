fn main() {
    println!("{}", solve(input()));
}

fn input() -> u32 {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.pop();
    let r: u32 = buf.parse().unwrap();
    r
}

fn solve(r: u32) -> f64 {
    let pi: f64 = std::f64::consts::PI;
    r as f64 * pi * 2 as f64
}
