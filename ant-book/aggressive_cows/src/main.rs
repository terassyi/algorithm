fn main() {
    println!("Hello, world!");
}

// C(d) = {任意の牛の間の間隔をd以上とすることができる}
fn c(n: usize, m: usize, x: &Vec<usize>, d: usize) -> bool {
    let mut last = 0;
    for i in 1..m {
        let mut crt = last+1;
        while crt < n && x[crt]-x[last] < d {
            crt += 1;
        }
        if crt == n { return false; }
        last = crt;
    }
    true
}

// 貪欲砲
// 牛舎の位置をソート
// 最初の牛をx0
// i番目の牛をxjに入れたら，i+1番目の牛はxj+d<=xkとなるような最小のxkをいれる
fn solve(n: usize, m: usize, x: &mut Vec<usize>) -> usize {
    x.sort();
    let mut lb = 0;
    let mut ub = 1000;
    while ub - lb > 1 {
        let mid = (ub + lb) / 2;
        if c(n, m, &x, mid) {
            lb = mid;
        } else {
            ub = mid;
        }
    }
    lb
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {
        assert_eq!(super::solve(5, 3, &mut vec![1,2,8,4,9]), 3)
    }
}
