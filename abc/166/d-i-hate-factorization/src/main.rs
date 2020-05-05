
fn main() {
    let ans = solve(input());
    println!("{} {}", ans.0, ans.1);
}

fn input() -> i64 {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.pop();
    let x: i64 = buf.parse().unwrap();
    x
}

fn solve(x: i64) -> (i64, i64) {
    for a in -118..120 {
        for b in -119..119 {
            if (a as i64).pow(5) - (b as i64).pow(5) == x {
                return (a, b);
            }
        }
    }
    (-1, -1)
}
