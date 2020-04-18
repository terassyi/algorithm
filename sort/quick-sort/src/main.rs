

fn main() {
    println!("Hello, world!");
}

fn partition<T: PartialOrd+Clone+Copy>(data: &mut Vec<T>, left: usize, right: usize) -> usize {
    let pivot = data[right];
    let mut index = left;
    let mut l = left;
    let r = right;

    while l < r {
        if data[l] < pivot {
            data.swap(l, index);
            index += 1;
        }
        l += 1;
    } 
    data.swap(index, right);
    index
}

fn quick_sort<T: PartialOrd+Clone+Copy>(data: &mut Vec<T>, left: usize, right: usize) {
    if left >= right {
        return;
    }
    let pivot = partition(data, left, right);
    if pivot <= 0 {
        return;
    }
    quick_sort(data, left, pivot - 1);
    quick_sort(data, pivot + 1, right);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_quick_sort() {
        let mut data = vec![3,1,3,6,2,9];
        let len = data.len() - 1;
        super::quick_sort(&mut data, 0, len);
        assert_eq!(data, vec![1,2,3,3,6,9]);
    }
}
