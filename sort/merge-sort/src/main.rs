fn main() {
    println!("Hello, world!");
}

fn merge_sort<T: PartialOrd+Clone+Copy>(data: &mut Vec<T>) -> Vec<T> {
    if data.len() <= 1 {
        return data.clone();
    }
    let (mut data1, mut data2) = devide(data);

    let tmp1 = merge_sort(&mut data1);
    let tmp2 = merge_sort(&mut data2);

    merge(&tmp1, &tmp2)
}

fn devide<T: PartialOrd+Clone+Copy>(data: &mut Vec<T>) -> (Vec<T>, Vec<T>) {
    let offset = data.len() / 2;
    let data1: Vec<T> = data.split_off(offset);
    (data.to_vec(), data1)
}

fn merge<T: PartialOrd+Clone+Copy>(data1: &Vec<T>, data2: &Vec<T>) -> Vec<T> {
    let mut merged: Vec<T> = Vec::with_capacity(data1.len() + data2.len());

    let mut left_offset: usize = 0;
    let mut right_offset: usize = 0;
    let mut merged_offset: usize = 0;

    while left_offset + right_offset < data1.len() + data2.len() {
        if data1[left_offset] <= data2[right_offset] {
            merged.push(data1[left_offset]);
            left_offset += 1;
        } else {
            merged.push(data2[right_offset]);
            right_offset += 1;
        }
        merged_offset += 1;
        if left_offset == data1.len() {
            while right_offset < data2.len() {
                merged.push(data2[right_offset]);
                right_offset += 1;
                merged_offset += 1;
            }
            break;
        }
        if right_offset == data2.len() {
            while left_offset < data1.len() {
                merged.push(data1[left_offset]);
                left_offset += 1;
                merged_offset += 1;
            }
            break;
        }
    }
    merged
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_devide() {
        let mut data = vec![1,2,3,4,5,6,7];
        let (data1, data2) = super::devide(&mut data);
        assert_eq!(data1, vec![1,2,3]);
        assert_eq!(data2, vec![4,5,6,7]);
    }

    #[test]
    fn test_merge() {
        let data1 = vec![1,3,8];
        let data2 = vec![2,6,7];
        assert_eq!(super::merge(&data1, &data2), vec![1,2,3,6,7,8]);
    }

    #[test]
    fn test_merge_sort() {
        let mut data: Vec<u32> = vec![3,1,5,9,7,2,0,8];
        assert_eq!(super::merge_sort(&mut data), vec![0,1,2,3,5,7,8,9]);
    }
}
