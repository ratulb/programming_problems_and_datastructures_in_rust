pub fn bubble_sort<T: PartialOrd>(array: &mut [T]) {
    for i in 0..array.len() {
        let mut swapped = false;
        for j in 0..array.len() - 1 - i {
            if array[j] > array[j + 1] {
                array.swap(j, j+1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::bubble_sort;
    #[test]
    fn bubble_sort_test() {
        let mut array = [10,2,100,56,9,11];
        bubble_sort(&mut array);
        assert!(array == [2,9,10,11,56,100]);
    }
}
