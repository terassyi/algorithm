fn main() {
    println!("Hello, world!");
}

fn solve(n: usize, lis: Vec<bool>) -> (usize, usize) {

}

fn calc(n: usize, k: usize, lis: Vec<bool>) -> usize {
    let mut f = vec![false; n];
    let mut res = 0;
    let mut sum = 0;
    
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {
        assert_eq!(super::solve(7, vec![false, false, true, false, true, false, false]), (3, 3))
    }
}
