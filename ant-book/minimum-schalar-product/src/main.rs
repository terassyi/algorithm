fn main() {
    println!("Hello, world!");
}

fn solve(v1: &mut Vec<i32>, v2: &mut Vec<i32>) -> i32 {
    v1.sort();
    v2.sort_by(|a, b| b.cmp(a));
    v1.iter().zip(v2.iter()).map(|(a, b)| a * b).sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {
        assert_eq!(super::solve(&mut vec![1,3,-5], &mut vec![-2,4,1]), -25)
    }
}
