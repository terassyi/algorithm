fn main() {
    println!("Hello, world!");
}

fn solve(n: usize, k: usize, l: Vec<f32>) -> f32 {
    let mut lb = 0.0;
    let mut ub = 1000000.0;
    for i in 0..100 {
        let mid = (lb+ub) / 2.0;
        if c(n, k, &l, mid) {
            lb = mid;
        } else {
            ub = mid;
        }
    }
    make_ans(ub)
}

fn c(n: usize, k: usize, l: &Vec<f32>, x: f32) -> bool {
    let mut num = 0;
    for i in 0..n {
        num += (l[i] / x) as usize;
    }
    num >= k
}

fn make_ans(x: f32) -> f32 {
    ((x * 100.0) as u32) as f32 / 100.0
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {
        assert_eq!(super::solve(4,11,vec![8.02,7.43,4.57,5.39]), 2.00)
    }
}
