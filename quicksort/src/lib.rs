/**
 * Sample implementation of quick sort alogrithm in rust for u16 type
 */

pub fn quicksort<'a>(array: &'a mut [u16]) -> &'a mut [u16] {
    sort(array, 0, array.len() - 1);
    array
}
fn sort(array: &mut [u16], start: usize, end: usize) {
    if start < end {
        let partition = partition(array, start, end);
        if array[..partition].len() > 1 {
            sort(array, start, partition - 1);
        }
        if array[partition + 1..].len() > 1 {
            sort(array, partition + 1, end);
        }
    }
}
fn partition(array: &mut [u16], left: usize, right: usize) -> usize {
    let pivot = array[right];
    let mut left_ptr = left;
    let mut right_ptr = right;

    loop {
        while left_ptr < right_ptr && array[left_ptr] <= pivot {
            left_ptr += 1;
        }
        right_ptr -= 1;
        while right_ptr > left_ptr && array[right_ptr] > pivot {
            right_ptr -= 1;
        }
        if left_ptr >= right_ptr {
            break;
        }
        swap(array, left_ptr, right_ptr);
    }
    swap(array, left_ptr, right);
    left_ptr
}

fn swap(array: &mut [u16], left_ptr: usize, right_ptr: usize) {
    let temp = array[left_ptr];
    array[left_ptr] = array[right_ptr];
    array[right_ptr] = temp;
}

#[cfg(test)]
mod tests {
    use super::quicksort;
    use rand::Rng;
    #[test]
    fn quicksort_test() {
        let mut runs = 5;
        loop {
            let mut array: [u16; 20] = [0; 20];
            rand::thread_rng().fill(&mut array);
            quicksort(&mut array);
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
