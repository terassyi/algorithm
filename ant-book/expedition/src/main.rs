use std::collections::BinaryHeap;

fn main() {
    println!("Hello, world!");
}

fn solve(n: usize, l: usize, p: u32, a: Vec<u32>, b: Vec<u32>) -> usize {
    let mut heap = BinaryHeap::new();
    
    let mut remain: u32 = p;

    let mut dis: u32 = 0;
    let mut ans = 0;
    for (place, quantity) in a.iter().zip(b.iter()) {
        dis += place;
        heap.push(quantity);
        if dis > remain {
            let supplyment: u32 = *heap.pop().unwrap();
            remain += supplyment;
            ans += 1;
        }
        if dis > l as u32 {
            break;
        }
    }
    ans as usize
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {
        let n: usize = 4;
        let l: usize = 25;
        let p: u32 = 10;

        let a: Vec<u32> = vec![10,14,20,21];
        let b: Vec<u32> = vec![10,5,2,4];

        assert_eq!(super::solve(n, l, p, a, b), 2);
    }
}
