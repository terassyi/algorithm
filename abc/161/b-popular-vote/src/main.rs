
fn main() {
    let (n, m, a) = input();
    println!("{}", solve(n, m, a));
}

fn input() -> (u32, u32, Vec<u32>) {
    let mut n_m = String::new();
    let mut a_vec = String::new();
    std::io::stdin().read_line(&mut n_m).unwrap();
    std::io::stdin().read_line(&mut a_vec).unwrap();
    n_m.pop();
    a_vec.pop();

    let mut n_m = n_m.split_whitespace();
    let aa = a_vec.split_whitespace();
    let n: u32 = n_m.next().unwrap().parse().unwrap();
    let m: u32 = n_m.next().unwrap().parse().unwrap();
    let a: Vec<u32> = aa.map(|x| x.parse().unwrap()).collect();
    (n, m, a)
}

fn solve(n: u32, m: u32, a: Vec<u32>) -> String {
    let sum = a.iter().sum();
    let mut counter = 0;
    for i in a.iter() {
        if i * 4 * m >= sum {
            counter += 1;
        }
    }
    if counter < m {
        return String::from("No");
    }
    String::from("Yes")
}
