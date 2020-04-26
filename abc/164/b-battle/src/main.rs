fn main() {
    let (a, b, c, d) = input();
    println!("{}", solve(a,b,c,d));
}

fn input() -> (i32, i32, i32, i32) {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let a: i32 = iter.next().unwrap().parse().unwrap();
    let b: i32 = iter.next().unwrap().parse().unwrap();
    let c: i32 = iter.next().unwrap().parse().unwrap();
    let d: i32 = iter.next().unwrap().parse().unwrap();
    (a,b,c,d)
}

fn solve(a: i32, b: i32, c: i32, d: i32) -> String {
    let mut takahashi: (i32, i32) = (a, b); // hp, attack
    let mut aoki: (i32, i32) = (c, d);

    loop {
        aoki.0 -= takahashi.1;
        if aoki.0 <= 0 {
            return String::from("Yes");
        }
        takahashi.0 -= aoki.1;
        if takahashi.0 <= 0 {
            return String::from("No");
        }
    }
}
