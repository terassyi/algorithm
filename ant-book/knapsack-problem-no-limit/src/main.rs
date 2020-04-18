use std::cmp;

fn main() {
    println!("Hello, world!");
}

fn solve_O_nW2(n: usize, W: usize, weight: Vec<u32>, value: Vec<u32>) -> u32 {
    let mut dp: Vec<Vec<u32>> = vec![vec![0u32; W+1]; n+1];

    for i in 0..n {
        for j in 0..W+1 {
            let mut k: u32 = 0;
            while k * weight[i] <= j as u32 {
                dp[i+1][j] = cmp::max(dp[i+1][j], dp[i][j-((k as usize) * (weight[i] as usize))]+ (k * value[i]));
                k += 1;
            }
        }
    }
    // println!("{:?}", dp);
    dp[n][W]
}

fn  solve_O_nW(n: usize, W: usize, weight: Vec<u32>, value: Vec<u32>) -> u32 {
    let mut dp = vec![vec![0u32; W+1]; n+1];
    for i in 0..n {
        for j in 0..W+1 {
            if j < weight[i] as usize {
                dp[i+1][j] = dp[i][j];
            } else {
                dp[i+1][j] = cmp::max(dp[i][j], dp[i+1][j- weight[i] as usize] + value[i]);
            }
        }
    }
    println!("{:?}", dp);
    dp[n][W]
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve_O_nW2() {
        let n: usize = 3;
        let weight: Vec<u32> = vec![3,4,2];
        let value: Vec<u32> = vec![4,5,3];
        let W: usize = 7;

        assert_eq!(super::solve_O_nW2(n, W, weight, value), 10);
        
    }

    #[test]
    fn test_solve_O_nW() {
        let n: usize = 3;
        let weight: Vec<u32> = vec![3,4,2];
        let value: Vec<u32> = vec![4,5,3];
        let W: usize = 7;

        assert_eq!(super::solve_O_nW(n, W, weight, value), 10);
        
    }
}
