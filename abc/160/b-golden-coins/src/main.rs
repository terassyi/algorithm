fn main() {
    let i = input();
    println!("{}", solve(i));
}

fn input() -> usize {
    let mut buf = String::from("");
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.pop();
    let i: usize = buf.parse().unwrap();
    i
}

fn solve(input: usize) -> usize {
    if input == 0 {
        return 0;
    }

    let number_500 = input / 500;
    let remain = input - (number_500 * 500);
    let number_5 = remain / 5;

    number_500 * 1000 + number_5 * 5
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve1() {
        assert_eq!(2020, super::solve(1024))
    }

    #[test]
    fn test_solve2() {
        assert_eq!(0, super::solve(0))
    }

    #[test]
    fn test_solve3() {
        assert_eq!(2000000000, super::solve(1000000000))
    }
}
