use std::cmp;

fn main() {
    println!("Hello, world!");
}

fn solve(n: usize, W: usize, weight: Vec<u32>, value: Vec<u32>) -> u32 {
    let mut memo: Vec<Vec<u32>> = vec![vec![0u32; W+1]; n+1];
    
    for (i, v) in value.iter().enumerate() {
        for (j, w) in weight.iter().enumerate() {
            memo[i+1][j] = cmp::max(memo[i+1][j], memo[i][j]);
            if j + (weight[i] as usize) <= W {
                memo[i+1][j + weight[i] as usize] = cmp::max(memo[i+1][j + weight[i] as usize], memo[i][j] + v);
            }
        }
    }
    println!("{:?}", memo);
    memo[n][W]
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {
        let n: usize = 4;
        let W: usize = 5;
        let weight: Vec<u32> = vec![2,1,3,2];
        let value: Vec<u32> = vec![3,2,4,2];

        assert_eq!(super::solve(n, W, weight, value), 7 as u32);
    }
}
