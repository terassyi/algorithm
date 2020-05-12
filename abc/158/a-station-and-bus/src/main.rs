fn main() {
    let s = input();
    println!("{}", solve(s));
}

fn input() -> Vec<char> {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.pop();
    let s = buf.chars();
    s.collect()
}

fn solve(s: Vec<char>) -> String {
    let mut a = 0;
    let mut b = 0;
    for c in s.iter() {
        if *c == 'A' { a += 1; }
        else if *c == 'B' { b += 1; }
    }
    if a != 0 && b != 0 {
        return String::from("Yes");
    }
    String::from("No")
}
