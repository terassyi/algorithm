use std::io::Read;

fn main() {
    let buf = input();
    println!("{}", solve(buf));
}

fn input() -> String {
    let mut buf = String::from("");
    std::io::stdin().read_to_string(&mut buf).unwrap();
    buf
}

fn solve(input: String) -> String {
    let yes = String::from("Yes");
    let no = String::from("No");

    let input = input.chars().collect::<Vec<char>>();

    if input[2] == input[3] && input[4] == input[5] {
        return yes;
    }
    no
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {
        let test = String::from("coffer");
        let result = super::solve(test);
        assert_eq!(result, "No");
    }
}
