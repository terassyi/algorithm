fn main() {
    let (n, c) = input();
    println!("{}", solve(n, c));
}

fn input() -> (u32, Vec<char>) {
    let mut buf1 = String::new();
    let mut buf2 = String::new();
    std::io::stdin().read_line(&mut buf1).unwrap();
    std::io::stdin().read_line(&mut buf2).unwrap();

    buf1.pop();
    buf2.pop();

    let n: u32 = buf1.parse().unwrap();
    let s: Vec<char> = buf2.chars().collect();
    (n, s)
}

fn solve(n: u32, s: Vec<char>) -> u32 {
    let mut R_index: Vec<usize> = Vec::new();
    let mut G_index: Vec<usize> = Vec::new();
    let mut B_index: Vec<usize> = Vec::new();

    for (i, c) in s.iter().enumerate() {
        match c {
            'R' => R_index.push(i),
            'G' => G_index.push(i),
            'B' => B_index.push(i),
            _ => {},
        }
    }
    let mut counter: u32 = 0;
    for r in R_index.iter() {
        for g in G_index.iter() {
            for b in B_index.iter() {
                let mut cmp_vec = vec![*r, *g, *b];
                cmp_vec.sort();
                if cmp_vec[1] as i32 - cmp_vec[0] as i32 == cmp_vec[2] as i32 - cmp_vec[1] as i32 {
                    continue;
                }
                counter += 1;
            }
        }
    }
    counter
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve1() {
        let n: u32 = 4;
        let s: Vec<char> = vec!['R', 'R', 'G', 'B'];
        assert_eq!(super::solve(n,s), 1);
    }

    #[test]
    fn test_solve2() {
        let n: u32 = 39;
        let s: Vec<char> = "RBRBGRBGGBBRRGBBRRRBGGBRBGBRBGBRBBBGBBB".to_string().chars().collect();
        assert_eq!(super::solve(n,s), 1800);
    }

}
