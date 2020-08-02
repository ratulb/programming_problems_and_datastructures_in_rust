pub fn selection_sort<T: PartialOrd>(array: &mut [T]) {
    for i in 0..array.len() - 1 {
        let mut min_index = i;
        for j in i + 1..array.len() {
            if array[j] < array[min_index] {
                min_index = j;
            }
        }
        if min_index == i {
            continue;
        } else {
            array.swap(i, min_index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::selection_sort;
    #[test]
    fn selection_sort_test() {
        let mut array = [10, 2, 100, 56, 9, 11,200, 11];
        selection_sort(&mut array);
        println!("{:?}", array);
        assert!(array == [2, 9, 10, 11, 11, 56, 100, 200]);
    }
}
