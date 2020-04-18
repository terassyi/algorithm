fn main() {
    println!("Hello, world!");
}

fn solve(coins: &mut [u32; 6], number_of_coins: &mut [usize; 6], a: u32) -> usize {
    let mut ans = 0;
    let mut b = a;
    coins.reverse();
    number_of_coins.reverse();
    let mut tuple = coins.iter().zip(number_of_coins);
    
    for (coin, number) in tuple {
        loop {
            if b < *coin || *number == 0 as usize {
                break;
            }
            b -= *coin;
            *number -= 1;
            ans += 1;
            println!("remain: {}, number: {}: sum: {}", b, number, ans);
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {
        let mut coins: [u32; 6] = [1, 5, 10, 50, 100, 500];
        let mut number_of_coins: [usize; 6] = [3, 2, 1, 3, 0, 2];
        let a: u32 = 620;
        let wanted: usize = 6;

        assert_eq!(super::solve(&mut coins, &mut number_of_coins, a), wanted);
    }
}
