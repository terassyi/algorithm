fn main() {
    println!("{:?}", solve2(input()));
}

fn input() -> Vec<u32> {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.pop();
    buf.chars().map(|c| c as u32 - 48).collect()
}

fn solve2(num: Vec<u32>) -> u32 {
    let mut counter = 0;
    for i in 0..num.len()-3 {
        let mut n: u32 = num[i] * 1000 + num[i+1] * 100 + num[i+2] * 10 + num[i+3];
        n = n % 2019;
        if n == 0 {
            counter += 1;
        }
        for k in num[i+4..].iter() {
            n = (n * 10 + k) % 2019;
            if n == 0 {
                counter += 1;
            }
        }
    }
    counter
}

#[cfg(test)]
mod tests {
    
    #[test]
    fn test_solve2() {
        let num: Vec<u32> = vec![1,8,1,7,1,8,1,7,1,2,1,1,4];
        assert_eq!(super::solve2(num), 3);
    }
}
