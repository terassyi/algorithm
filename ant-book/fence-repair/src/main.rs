use std::collections::BinaryHeap;

fn main() {
    println!("Hello, world!");
}

fn solve(n: u32, lis: Vec<u32>) -> u32 {
    let mut heap = BinaryHeap::new();

    for a in lis.iter() {
        heap.push(-1 * (*a as i32));
    }
    let mut ans: i32 = 0;
    let mut size: usize = heap.len();
    while size > 1 {
        let a = heap.pop().unwrap();
        let b = heap.pop().unwrap();

        ans += -1 * (a + b);
        heap.push(a + b);
        size -= 1;
    }
    ans as u32
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {
        let lis: Vec<u32> = vec![8,5,8];
        let n: u32 = 3;

        assert_eq!(super::solve(n, lis), 34);
    }
}
