fn main() {
    let s = input();
    println!("{}", solve(s));
}

fn input() -> Vec<u8> {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.pop();
    buf.into_bytes()
}

fn solve(s: Vec<u8>) -> String {
    let n = s.len();
    if !is_plaindrome(&s) { return String::from("No"); }
    if !is_plaindrome(&s[..(n-1)/2].to_vec()) { return String::from("No"); }
    if !is_plaindrome(&s[((n+3)/2)-1..].to_vec()) { return String::from("No"); }
    String::from("Yes")
}

fn is_plaindrome(s: &Vec<u8>) -> bool {
    let l = s.len();
    let mut first: Vec<u8> = s.to_vec();
    let latter: Vec<u8> = first.split_off(l/2);
    let mut latter_iter = latter.iter();
    if s.len() % 2 != 0 {
        latter_iter.next().unwrap();
    }
    first.reverse();
    for (a, b) in first.iter().zip(latter_iter) {
        if a != b {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_is_plaindrome() {
        assert_eq!(super::is_plaindrome(&"level".as_bytes().to_vec()), true);
    }
    #[test]
    fn test_solve() {
        assert_eq!(super::solve("akasaka".as_bytes().to_vec()), String::from("Yes"));
    }
    #[test]
    fn test_solve2() {
        assert_eq!(super::solve("level".as_bytes().to_vec()), String::from("No"));
    }
}
