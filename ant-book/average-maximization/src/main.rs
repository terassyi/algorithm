fn main() {
    println!("Hello, world!");
}

// C(x) = 単位重さ当たりの価値がx以上となるように選ぶことができる
// C(x) = (vi - x * wi)の大きい方からk個の和が0以上
fn c(n: usize, k: usize, lis: &Vec<(f32, f32)>, x: f32) -> bool {
    let mut y = vec![0f32; n];
    for (i, (w, v)) in lis.iter().enumerate() {
        y[i] = v - x * w;
    }
    let mut sum = 0f32;
    y.sort_by(|a, b| b.partial_cmp(&a).unwrap());
    for i in 0..k {
        sum += y[i];
    }
    sum >= 0.0
}

fn solve(n: usize, k: usize, lis: Vec<(f32, f32)>) -> f32 {
    let mut lb: f32 = 0.0;
    let mut ub: f32 = 1000.0;
    for _ in 0..100 {
        let mid = (lb+ub)/2.0;
        if c(n, k, &lis, mid) {
            lb = mid;
        } else {
            ub = mid;
        }
    }
    (ub * 100.0) / 100.0
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {
        let lis: Vec<(f32, f32)> = vec![(2.0,2.0),(5.0,3.0),(2.0,1.0)];
        assert_eq!(super::solve(3,2,lis), 0.75);
    }
}
