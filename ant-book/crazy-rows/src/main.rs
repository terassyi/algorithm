fn main() {
    println!("Hello, world!");
}

fn solve(matrix: Vec<Vec<u8>>) -> u32 {
    let mut a: Vec<u8> = matrix.iter().map(|row| (row.len()- 1 - row.iter().rev().enumerate().find(|(i, v)| **v == 1).unwrap().0) as u8).collect();
    let mut ans = 0;
    for i in 0..a.len() {
        let mut pos: usize = 50; 
        for j in 1..a.len() {
            if a[j] <= i as u8 {
                pos = j;
                break;
            }
        }
        while pos > i {
            a.swap(pos, pos-1);
            pos -= 1;
            ans += 1;
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve1() {
        let matrix = vec![
            vec![0,0,1],
            vec![1,0,0],
            vec![0,1,0],
        ];
        assert_eq!(super::solve(matrix), 2);
    }
    #[test]
    fn test_solve2() {
        let matrix = vec![
            vec![1,1,1,0],
            vec![1,1,0,0],
            vec![1,1,0,0],
            vec![1,0,0,0],
        ];
        assert_eq!(super::solve(matrix), 4);
    }
}
