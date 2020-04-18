fn main() {
    println!("Hello, world!");
}

fn bubble_sort<T: PartialOrd>(data: &mut Vec<T>) {
    for i in 0..data.len() {
        for j in 0..data.len()-i-1 {
            if data[j] > data[j+1] {
                data.swap(j+1, j);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_bubble_sort() {
        let mut data = vec![10,8,4,1,0,7,100,3,4,1,9,5];
        super::bubble_sort(&mut data);

        assert_eq!(data, vec![0,1,1,3,4,4,5,7,8,9,10,100]);
    }
}
