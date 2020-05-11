fn main() {
    let (n, k, a) = input();
    println!("{}", solve(n, k, a));
}

fn input() -> (usize, usize, Vec<usize>) {
    let mut buf1 = String::new();
    let mut buf2 = String::new();
    std::io::stdin().read_line(&mut buf1).unwrap();
    std::io::stdin().read_line(&mut buf2).unwrap();
    buf1.pop();
    buf2.pop();
    let mut iter1 = buf1.split_whitespace();
    let n: usize = iter1.next().unwrap().parse().unwrap();
    let k: usize = iter1.next().unwrap().parse().unwrap();
    let iter2 = buf2.split_whitespace();
    let a: Vec<usize> = iter2.map(|s| s.parse().unwrap() ).collect();
    (n, k, a)
}

fn solve(n: usize, k: usize, a: Vec<usize>) -> usize {
    let mut visited: Vec<bool> = vec![false; n];
    let mut now = 0;
    let mut counter = 0;
    visited[now] = true;
    loop {
        let next = a[now] - 1;
        counter += 1;
        now = next;
        if k == counter { return next; }
        if visited[next] {
            break; 
        }
        visited[now] = true;
    }
    let cycle_start = now;
    let mut before_loop = 0;
    now = 0;
    loop {
        if now == cycle_start {
            break;
        }
        let next = a[now] - 1;
        before_loop += 1;
        now = next;
    }
    let cycle = counter - before_loop;
    let tele = before_loop + (k - before_loop) % cycle;
    let mut next = 0;
    for _ in 0..tele {
        next = a[next] - 1;
    }
    next + 1
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {
        assert_eq!(super::solve(4,5,vec![3,2,4,1]),4);
    }

    #[test]
    fn test_solve2() {
        assert_eq!(super::solve(6, 727202214173249351, vec![6,5,2,5,3,2]), 2);
    }
}
