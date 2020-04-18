fn main() {
    println!("Hello, world!");
}

fn scheduler(start: &mut Vec<u32>, end: &mut Vec<u32>, tasks: u32) -> u64 {
    // 終了時刻が早いものを選ぶ
    
    let mut ans = 0;
    let mut current_end: u32 = 0;

    for (start, end) in start.iter().zip(end) {
        if current_end <= *start {
            ans += 1;
            current_end = *end;
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_task_scheduling() {
        let mut start: Vec<u32> = vec![1,2,4,6,8];
        let mut end: Vec<u32> = vec![3,5,7,9,10];
        let tasks = 5;

        assert_eq!(super::scheduler(&mut start, &mut end, tasks), 3);
    }
}
