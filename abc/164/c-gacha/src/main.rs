use std::collections::HashMap;

fn main() {
    let (n, v) = input();
    println!("{}", solve(n, v));
}

fn input() -> (usize, Vec<String>) {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.pop();
    let n: usize = buf.parse().unwrap();
    let mut v: Vec<String> = Vec::new();
    for i in 0..n {
        let mut b = String::new();
        std::io::stdin().read_line(&mut b).unwrap();
        b.pop();
        v.push(b);
    }
    (n, v)
}

fn solve(n: usize, v: Vec<String>) -> usize {
    let mut set: HashMap<String, bool> = HashMap::new();
    for a in v.iter() {
        set.entry(a.to_string()).or_insert(true);
    }
    set.len()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {
        let n: usize = 4;
        let v: Vec<String> = vec![String::from("aaaa"), String::from("a"), String::from("aaa"), String::from("aa")];
        assert_eq!(super::solve(n, v), 4);
    }
}
