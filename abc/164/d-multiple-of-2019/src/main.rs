fn main() {
    println!("{:?}", solve(input()));
}

fn input() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.pop();
    buf
}

fn solve(num: String) -> u64 {
    let mut counter: u64 = 0;
    for i in 0..num.len()-3 {
        for j in i+4..num.len()+1 {
            let n: u64 = num[i..j].parse().unwrap();
            
            if n % 2019 == 0 {
                counter += 1;
            }
        }
    }
    counter
}

fn solve2(num: String) -> u64 {
    let mut counter = 0;
    for i in 0..num.len()-3 {
        let mut n: u64 = num[i..i+4].parse().unwrap();
        n = n % 2019;
        if n == 0 {
            counter += 1;
        }
        for c in num[i+4..].chars() {
            let k: u64 = (c as i32 - 48) as u64;
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
    fn test_solve1() {
        let num = String::from("1817181712114");
        assert_eq!(super::solve(num), 3);
    }

    #[test]
    fn test_solve2() {
        let num = String::from("14282668646");
        assert_eq!(super::solve2(num), 2);
    }
}
