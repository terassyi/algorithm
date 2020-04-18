fn main() {
    let (n, x, y): (i32, i32, i32) = input();
    let ans = solve(n, x, y);
    for a in ans.iter() { println!("{}", a); }
}

fn input() -> (i32, i32, i32) {
    let mut buf = String::from("");
    buf.pop();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let n: i32 = iter.next().unwrap().parse().unwrap();
    let x: i32 = iter.next().unwrap().parse().unwrap();
    let y: i32 = iter.next().unwrap().parse().unwrap();
    (n, x, y)
}

fn solve(n: i32, x: i32, y: i32) -> Vec<usize> {
    let mut ans: Vec<usize> = vec![0usize; (n-1) as usize];
    for i in 1..n {
        for j in (i+1)..(n+1) {
            // min{|j − i|, |X − i| + 1 + |j − Y |, |Y − i| + 1 + |j − X|}
            let dis = std::cmp::min(j - i, (i - x).abs() + 1 + (j - y).abs()) as usize;
            ans[dis-1] += 1 as usize;
        }
    }
    ans
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_solve1() {
        let n: i32 = 5;
        let x: i32 = 2;
        let y: i32 = 4;
        let wanted: Vec<usize> = vec![5,4,1,0];
        assert_eq!(super::solve(n,x,y), wanted);
    }

    #[test]
    fn test_solve2() {
        let n: i32 = 10;
        let x: i32 = 4;
        let y: i32 = 8;
        let wanted: Vec<usize> = vec![10,12,10,8,4,1,0,0,0];
        assert_eq!(super::solve(n,x,y), wanted); 
    }
}
