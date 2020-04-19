use std::io::Read;

fn main() {
    let (x, y, z) = input();
    let ans = solve(x, y, z);
    println!("{} {} {}", ans.0, ans.1, ans.2);
}

fn input() -> (u32, u32, u32) {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    buf.pop();
    let mut iter = buf.split_whitespace();
    let x: u32 = iter.next().unwrap().parse().unwrap();
    let y: u32 = iter.next().unwrap().parse().unwrap();
    let z: u32 = iter.next().unwrap().parse().unwrap();
    (x, y, z)
}

fn solve(x: u32, y: u32, z: u32) -> (u32, u32, u32) {
    (z, x, y)
}
