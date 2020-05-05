fn main() {
    let s = input();
    println!("{}", solve(&s));
}

fn input() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.pop();
    buf
}

fn solve(s: &str) -> String {
    if s.chars().next().unwrap() == 'B' {
        return String::from("ARC");
    }
    String::from("ABC")
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {
        assert_eq!(super::solve("ABC"), String::from("ARC"));
    }
    #[test]
    fn test_solve2() {
        assert_eq!(super::solve("ARC"), String::from("ABC"));
    }
}
