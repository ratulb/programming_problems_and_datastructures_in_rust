pub fn selection_sort(array: &mut [u8]) {
    //Iterate through each element of the array till the last to last element
    for i in 0..array.len() - 1 {
        //Current element position would be the position for next minimum element
        let mut min_index = i;
        for j in i + 1..array.len() {
            //Compare current minimum element with each next element till the last element of
            //the array - swap the index of next element with min_index if following condition holds true
            if array[j] < array[min_index] {
                min_index = j;
            }
        }
        //Swap ith element of the outer loop with element at min_index - if required
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
        let mut array = [10, 2, 100, 56, 9, 11, 11];
        selection_sort(&mut array);
        assert!(array == [2, 9, 10, 11, 11, 56, 100]);
    }
}
