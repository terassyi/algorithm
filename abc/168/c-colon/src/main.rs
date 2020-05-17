fn main() {
    let (a, b, h, m) = input();
    println!("{:?}", solve(a, b, h, m));
}

fn input() -> (f64, f64, f64, f64) {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.pop();
    let mut iter = buf.split_whitespace();
    let a: f64 = iter.next().unwrap().parse().unwrap();
    let b: f64 = iter.next().unwrap().parse().unwrap();
    let h: f64 = iter.next().unwrap().parse().unwrap();
    let m: f64 = iter.next().unwrap().parse().unwrap();
    (a, b, h, m)
}

fn solve(a: f64, b: f64, h: f64, m: f64) -> f64 {
    let m_angle = m * 6.0;
    let h_angle = h * 30.0 + m / 2.0;
    let mut theta = (h_angle - m_angle);
    if theta > 180.0 {
        theta = 360.0 - theta;
    }
    theta = theta * std::f64::consts::PI / 180.0;
    // println!("theta_h={:?} theta_m={:?} theta={:?} cos(theta)={:?}", h_angle, m_angle, theta, theta.cos());
    let c_pow = a * a + b * b - 2.0 * b * a * theta.cos();
    ((c_pow.sqrt() * 1000000000.0) as i64) as f64 / 1000000000.0
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {
        assert_eq!(super::solve(3.0,4.0,9.0,0.0), 5.0);
    }
    #[test]
    fn test_solve2() {
        assert_eq!(super::solve(3.0,4.0,10.0,40.0), 4.56425719433005567605);
    }
}