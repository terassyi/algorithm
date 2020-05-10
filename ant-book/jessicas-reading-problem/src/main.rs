use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

fn solve(p: usize, a: Vec<usize>) -> usize {
    // 事柄の種類
    let k = kinds(&a);
    // 尺取法
    let mut s = start_index(k, &a);
    let mut t = 0;
    let mut res = s-t+1;

    let mut map = HashMap::new();
    for i in t..s+1 {
        let c = map.entry(a[i]).or_insert(0);
        *c += 1;
    }
    loop {
        while t < p && is_filled(k, &map) {
            t += 1;
            let c = map.entry(a[t]).or_insert(0);
            *c -= 1;
        }
        println!("t={} s={} res={}", t, s, res);
        if s == a.len()-1 {
            break;
        }
        s += 1;
        let c = map.entry(a[s]).or_insert(0);
        *c += 1;
        res = std::cmp::min(res, s-t+1);
        
    }
    res
}

fn start_index(k: usize, a: &Vec<usize>) -> usize {
    let mut hash_map: HashMap<usize, usize> = HashMap::new();
    for (i, v) in a.iter().enumerate() {
        let c = hash_map.entry(*v).or_insert(0);
        *c += 1;
        if is_filled(k, &hash_map) { return i; }
    }
    a.len()
}

fn kinds(a: &Vec<usize>) -> usize {
    let mut hash_map: HashMap<usize, usize> = HashMap::new();
    for v in a.iter() {
        let c = hash_map.entry(*v).or_insert(0);
        *c += 1;
    }
    hash_map.len()
}

fn is_filled(size: usize, map: &HashMap<usize, usize>) -> bool {
    if size != map.len() { return false; }
    for (_, v) in map.iter() {
        if *v == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {
        assert_eq!(super::solve(5, vec![1,8,8,8,1]), 2);
    }
}
