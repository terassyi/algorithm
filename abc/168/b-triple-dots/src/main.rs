fn main() {
    let (k, s) = input();
    println!("{}", solve(k, s));
}

fn input() -> (usize, String) {
    let mut buf1 = String::new();
    let mut buf2 = String::new();
    std::io::stdin().read_line(&mut buf1).unwrap();
    std::io::stdin().read_line(&mut buf2).unwrap();
    buf1.pop();
    buf2.pop();
    let k: usize = buf1.parse().unwrap();
    (k, buf2)
}

fn solve(k: usize, s: String) -> String {
    if k >= s.len() {
        return s;
    }
    format!("{}{}", &s[..k].to_string(), String::from("..."))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {
        let s = String::from("nikoandsolstice");
        assert_eq!(super::solve(7, s), String::from("nikoand..."));
    }
}