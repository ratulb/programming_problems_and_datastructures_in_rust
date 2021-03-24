///Merge two sorted arrays. One array will have more or enough space
///to accomodate the merged result. Extra spaces on the bigger array
///are filled with i32::MAX

pub fn merge_sorted_arrays<'a>(array1: &'a mut [i32], array2: &'a mut [i32]) -> &'a [i32] {
    let shorter: &mut [i32];
    let longer: &mut [i32] = if array1.len() > array2.len() {
        shorter = array2;
        array1
    } else {
        shorter = array1;
        array2
    };
    let mut i: i32 = (shorter.len() - 1) as i32;
    let mut j: i32 = (first_index_of_max(longer) - 1) as i32;
    let mut k = i + j + 1;
    while i >= 0 && j >= 0 {
        if shorter[i as usize] < longer[j as usize] {
            longer[k as usize] = longer[j as usize];
            j -= 1;
        } else {
            longer[k as usize] = shorter[i as usize];
            i -= 1;
        }
        k -= 1;
    }
    while i >= 0 {
        longer[k as usize] = shorter[i as usize];
        i -= 1;
    }
    while j >= 0 {
        longer[k as usize] = longer[j as usize];
        j -= 1;
    }
    longer
}

fn first_index_of_max(array: &[i32]) -> usize {
    let mut low = 0;
    let mut index_of_max = array.len();
    let mut high = index_of_max - 1;

    while low <= high {
        let mid = low + (high - low) / 2;
        if array[mid] == i32::MAX {
            index_of_max = mid;
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    index_of_max
}

#[cfg(test)]
mod tests {
    use super::merge_sorted_arrays;
    #[test]
    fn test_merge_sorted_arrays() {
        let array1 = &mut [1, 4, 7, 24, 27, 76, 78];
        let max = i32::MAX;
        let array2 = &mut [3, 9, 11, 19, 34, 200, max, max, max, max, max, max, max];
        assert_eq!(
            merge_sorted_arrays(array1, array2),
            &[1, 3, 4, 7, 9, 11, 19, 24, 27, 34, 76, 78, 200]
        );
    }
}
