fn main() {
    let n = input();
    println!("{}", solve(n));
}

fn input() -> usize {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.pop();
    let n: usize = buf.parse().unwrap();
    n
}

fn solve(n: usize) -> String {
    let l = n % 10;
    if l == 2 || l == 4 || l == 5 || l == 7 || l == 9 {
        return String::from("hon");
    } else if l == 0 || l == 1 || l == 6 || l == 8 {
        return String::from("pon");
    }
    String::from("bon")
}