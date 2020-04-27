fn main() {
    println!("Hello, world!");
}

fn solve(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    let a = p2.0 - p1.0;
    let b = p2.1 - p1.1;
    let g = gcd(a, b);
    let l = a/g;
    a/l - 1
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {
        let p1: (i32, i32) = (1,11);
        let p2: (i32, i32) = (5,3);
        assert_eq!(super::solve(p1, p2), 3);
    }
}
