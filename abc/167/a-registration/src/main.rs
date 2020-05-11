fn main() {
    let (s, t) = input();
    println!("{}", solve(&s, &t));
}

fn input() -> (String, String) {
    let mut buf1 = String::new();
    let mut buf2 = String::new();
    std::io::stdin().read_line(&mut buf1).unwrap();
    std::io::stdin().read_line(&mut buf2).unwrap();
    buf1.pop();
    buf2.pop();
    (buf1, buf2)
}

fn solve(s: &str, t: &str) -> String {
    if t.len() - s.len() != 1 { return String::from("No"); }
    let s_vec = s.as_bytes().to_vec();
    let t_vec = t.as_bytes().to_vec();
    for i in (0..s.len()) {
        if s_vec[i] != t_vec[i] { return String::from("No"); }
    }
    String::from("Yes")
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {
        assert_eq!(super::solve("chokudai", "chokudaiz"), String::from("Yes"));
    }
}
