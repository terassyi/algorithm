use std::collections::VecDeque;
use std::io::Read;

fn main() {
    let n = input();
    println!("{}", solve(n));
}

fn input() -> u64 {
    let mut buf = String::from("");
    std::io::stdin().read_to_string(&mut buf).unwrap();
    buf.pop();
    let n: u64 = buf.parse().unwrap();
    n
}

fn solve(n: u64) -> u64 {
    let mut queue: VecDeque<u64> = VecDeque::new();
    for i in 1..10 {
        queue.push_back(i as u64);
    }
    let mut counter = 9;
    let mut ans: u64 = 0;
    loop {
        let q = queue.pop_front().unwrap();
        let k = q % 10;
        if k != 0 {
            ans = 10 * q + k -1;
            queue.push_back(ans);
            counter += 1;
        }
        if counter == n { break; }
        ans = 10 * q + k;
        queue.push_back(ans);
        counter += 1;
        if counter == n { break; }
        if k != 9 {
            ans = 10 * q + k + 1;
            queue.push_back(ans);
            counter += 1;
        }
        if counter == n { break; }
    }
    ans
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {
        assert_eq!(super::solve(15 as u64), 23);
    }
}
