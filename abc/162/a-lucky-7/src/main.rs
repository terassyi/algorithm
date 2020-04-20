fn main() {
    let num = input();
    println!("{}", solve(num));
}

fn input() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.pop();
    buf
}

fn solve(num: String) -> String {
    for c in num.chars() {
        if c == '7' { return String::from("Yes"); }
    }
    String::from("No")
}
