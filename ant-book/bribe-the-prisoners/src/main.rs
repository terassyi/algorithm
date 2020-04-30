fn main() {
    let ans = solve(20,3, &mut vec![3,6,14]);
    println!("ans: {}", ans);
}

fn solve(p: usize, q: usize, a: &mut Vec<usize>) -> usize {
    // 動的計画法
    // dp[i][j]: i人目の囚人からj人目の囚人までの部分になったときに，
    // その中の囚人を全て解放するのに必要な最小枚数
    let mut b: Vec<usize> = vec![0];
    b.append(a);
    b.push(p+1);
    println!("b: {:?}", b);
    let mut dp = vec![vec![0usize; q+2]; q+1];
    println!("dp: {:?}", dp);
    //
    for w in 2..q+2 {
        let mut i: usize = 0;
        while i+w <= q+1 {
            let j: usize = i+w;
            let mut t: usize = 10000;
            for k in i+1..j {
                t = std::cmp::min(t, dp[i][k]+dp[k][j]);
            }
            dp[i][j] = t + b[j] - b[i] - 2;
            i += 1;
            show_dp_table(i, j, &dp);
        }
        
    }
    dp[0][q+1]
}

fn show_dp_table(i: usize, j: usize, dp: &Vec<Vec<usize>>) {
    println!("----dp table({}, {})----", i, j);
    for v in dp.iter() {
        println!("{:?}", v)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {
        assert_eq!(super::solve(20,3, &mut vec![3,6,14]), 35);
    }
}
