/***
 * Segregate posivtive and negative numbers - negatives are expected first in the result
 ***/

pub fn segregate(arr: &mut [i32]) -> &[i32] {
    if arr.len() == 0 || arr.len() == 1 {
        return arr;
    }
    let mut i = 0;
    let mut j = arr.len() - 1;
    while i < j {
        while i < arr.len() - 1 && arr[i] < 0 {
            i += 1;
        }
        while j > 0 && arr[j] >= 0 {
            j -= 1;
        }
        if i >= j {
            break;
        }
        let temp = arr[i];
        arr[i] = arr[j];
        arr[j] = temp;
        i += 1;
        j -= 1;
    }
    arr
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn segregate_pos_and_neg_test_1() {
        let mut arr = [-1, 3, 8, -4, 5, -6, 7, -20, 30, 40];
        segregate(&mut arr);
        for i in 0..4 {
            assert!(arr[i] < 0);
        }
        for i in 4..arr.len() {
            assert!(arr[i] >= 0);
        }
    }
}
