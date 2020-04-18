fn main() {
    println!("Hello, world!");
}

fn selection_sort<T: PartialOrd>(data: &mut Vec<T>) {
    for i in 0..(data.len()-1) {
        let mut min_offset = i;
        for j in (i+1)..data.len() {
            if data[j] < data[min_offset] {
                min_offset = j;
            }
        }
        if min_offset != i {
            data.swap(i, min_offset);
        }
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn test_selection_sort() {
        let mut data = vec![4,3,1,6,8,7,9];
        super::selection_sort(&mut data);
        assert_eq!(data, vec![1,3,4,6,7,8,9]);
    }
}
