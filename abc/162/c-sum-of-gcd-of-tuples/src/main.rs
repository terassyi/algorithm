fn main() {
    println!("{}", solve(input()));
}

fn input() -> u32 {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.pop();

    let n: u32 = buf.parse().unwrap();
    n
}

fn solve(n: u32) -> u64 {
    let mut ans: u64 = 0;
    for i in 1..n+1 {
        for j in 1..n+1 {
            for k in 1..n+1 {
                let a = gcd(gcd(i, j), k) as u64;
                ans += a;
            }
        }
    }
    ans
}

fn gcd(a: u32, b: u32) -> u32 {
    if a % b == 0 {
        b
    } else {
        gcd(b, a % b)
    }
}
