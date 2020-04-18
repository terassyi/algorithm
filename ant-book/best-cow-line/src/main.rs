

fn main() {
    println!("Hello, world!");
}

fn pop_front_push_back(src: &str, dst: &str) -> (String, String) {
    let poped = src.chars().next().unwrap();
    let pushed = dst.to_string() + &poped.to_string();

    (src[1..].to_string(), pushed)
}

fn pop_back_push_front(src: &str, dst: &str) -> (String, String) {
    let poped = src.to_string().pop().unwrap();
    let pushed = dst.to_string() + &poped.to_string();

    (src[0..((src.len()-1) as usize)].to_string(), pushed.to_string())
}

fn solve(n: u32, src: &str) -> String {
    let mut t = String::from("");
    let length = src.len();
    let mut s = src.to_string();
    for i in 0..length {
        let head = &s.chars().next().unwrap();
        let tail = &s.to_string().pop().unwrap();

        let (ss, tt) = match head < tail {
            true => pop_front_push_back(&s, &t),
            false => pop_back_push_front(&s, &t),
        };
        s = ss;
        t = tt;
    }
    t.to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_pop_front_push_back() {
        let src = "abcd";
        let dst = "efg";

        let (s, d) = super::pop_front_push_back(&src, &dst);
        assert_eq!(s, "bcd");
        assert_eq!(d, "efga");
    }

    #[test]
    fn test_pop_back_push_front() {
        let src = "abcd";
        let dst = "efg";

        let (s, d) = super::pop_back_push_front(&src, &dst);
        assert_eq!(s, "abc");
        assert_eq!(d, "efgd");
    }

    #[test]
    fn test_solve() {
        let n: u32 = 6;
        let s = "ACDBCB";
        let wanted = "ABCBCD";
        assert_eq!(super::solve(n, &s), wanted);
    }
}
