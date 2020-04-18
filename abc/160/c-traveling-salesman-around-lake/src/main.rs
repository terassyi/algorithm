fn main() {
    let (lake, house, positions) = input();
    println!("{}", solve(lake, house, positions));
}

fn input() -> (usize, usize, Vec<usize>) {
    let mut input1 = String::from("");
    let mut input2 = String::from("");

    std::io::stdin().read_line(&mut input1).unwrap();
    std::io::stdin().read_line(&mut input2).unwrap();

    input1.pop();
    input2.pop();

    let mut iter1 = input1.split_whitespace();
    let mut iter2 = input2.split_whitespace();

    let lake: usize = iter1.next().unwrap().parse().unwrap();
    let house: usize = iter1.next().unwrap().parse().unwrap();

    let positions: Vec<usize> = iter2.map(|i| i.parse().unwrap()).collect();

    (lake, house, positions)
}

fn solve(lake: usize, house: usize, positions: Vec<usize>) -> usize {
    let mut dis = vec![0usize; house];
    for i in 0..house-1 {
        dis[i] = positions[i+1] - positions[i];
    }
    dis[house-1] = positions[0] + (lake - positions[house-1]);
    let sum: usize = dis.iter().sum();
    let max = dis.iter().max().unwrap();
    sum - max
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {
        let lake: usize = 20;
        let house: usize = 3;
        let positions: Vec<usize> = vec![5, 10, 15];
        assert_eq!(super::solve(lake, house, positions), 10);
    }
}
