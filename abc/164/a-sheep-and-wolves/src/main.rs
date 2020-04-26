fn main() {
    let (s, w) = input();
    println!("{}", solve(s, w))
}

fn input() -> (u32, u32) {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.pop();
    let mut iter = buf.split_whitespace();
    let s: u32 = iter.next().unwrap().parse().unwrap();
    let w: u32 = iter.next().unwrap().parse().unwrap();
    (s, w)
}

fn solve(s: u32, w: u32) -> String {
    if s <= w { return String::from("unsafe"); }
    String::from("safe")
}
