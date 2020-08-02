pub fn insertion_sort<T: PartialOrd+Copy> (array: &mut [T]) {
    for i in 0..array.len() {
        let current = array[i];
        let mut j = i;
        while j > 0 && current < array[j-1] {
            array[j] = array[j-1];
            j -= 1;
        }
        array[j] = current;
    }
}
#[cfg(test)]
mod tests {
    use super::insertion_sort;
    #[test]
    fn insertion_sort_test() {
        let mut array = [10,2,100,56,9,11,11];
        insertion_sort(&mut array);
        assert!(array == [2,9,10,11,11,56,100]);
    }
}