fn main() {
    println!("Hello, world!");
}

fn solve(n: usize, a: Vec<usize>, k: usize) -> usize {
    search(0, n-1, &a, k)
}

fn search(start: usize, end: usize, a: &Vec<usize>, k: usize) -> usize {
    println!("start: {} end :{}", start, end);
    let mid = (end - start + 1) / 2;
    if mid == start { return start; }
    if mid == end { return end; }
    if a[mid] < k {
        return search(mid, (end - mid + 1) / 2 + mid, &a, k);
    }
    search(mid - (mid - start + 1) / 2 , mid, &a, k)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {
        assert_eq!(super::solve(5, vec![2,3,3,5,6], 6), 1);
    }
}
