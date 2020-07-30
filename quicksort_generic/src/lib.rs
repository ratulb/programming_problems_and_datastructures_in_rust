pub fn quicksort_generic<T: PartialOrd + Copy>(array: &mut [T]) {
    sort(array, 0, array.len() - 1);
}
fn partition<T: PartialOrd + Copy>(array: &mut [T], left: usize, right: usize) -> usize {
    let pivot = array[right];
    let mut i = left;
    for j in left..right {
        if array[j] <= pivot {
            array.swap(i, j);
            i += 1;
        }
    }
    array.swap(i, right);
    i
}
fn sort<T: PartialOrd + Copy>(array: &mut [T], left: usize, right: usize) {
    if left < right {
        let partition = partition(array, left, right);
        if array[..partition].len() > 1 {
            sort(array, left, partition - 1);
        }
        if array[partition + 1..].len() > 1 {
            sort(array, partition + 1, right);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::quicksort_generic;
    use rand::Rng;
    #[test]
    fn quicksort_test() {
        let mut runs = 5;
        loop {
            let mut array: [u16; 20] = [0; 20];
            rand::thread_rng().fill(&mut array);
            quicksort_generic(&mut array);
            if !is_sorted(&array) {
                panic!("Array is not sorted...");
            }
            println!("The sorted array is : {:?} ", array);
            runs -= 1;
            if runs == 0 {
                break;
            }
        }
    }
    fn is_sorted(array: &[u16]) -> bool {
        for idx in 1..array.len() {
            if array[idx - 1] > array[idx] {
                return false;
            }
        }
        true
    }
}
