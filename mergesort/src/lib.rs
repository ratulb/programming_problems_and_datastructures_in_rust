/**
 * A simple merge sort routine
 */

pub fn mergesort<'a>(array: &'a mut [u16]) -> &'a [u16] {
    sort(array, 0, array.len() - 1);
    array
}

pub fn merge(array: &mut [u16], low: usize, mid: usize, high: usize) {
    let aux: Vec<u16> = array.to_vec();

    let mut i = low;
    let mut j = mid + 1;

    for k in low..=high {
        if i > mid {
            array[k] = aux[j];
            j += 1;
        } else if j > high {
            array[k] = aux[i];
            i += 1;
        } else if aux[j] < aux[i] {
            array[k] = aux[j];
            j += 1;
        } else {
            array[k] = aux[i];
            i += 1;
        }
    }
}
pub fn sort(array: &mut [u16], low: usize, high: usize) {
    if high <= low {
        return;
    }
    let mid = low + (high - low) / 2;
    sort(array, low, mid);
    sort(array, mid + 1, high);
    merge(array, low, mid, high);
}

#[cfg(test)]
mod tests {
    use super::mergesort;
    use rand::Rng;
    #[test]
    fn merge_sort_test() {
        let mut runs = 5;
        loop {
            let mut array: [u16; 20] = [0; 20];
            rand::thread_rng().fill(&mut array);
            mergesort(&mut array);
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
