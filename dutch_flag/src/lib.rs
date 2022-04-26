/***
 * Sort 0, 1, 2
 * **/

pub fn arrange<'a>(arr: &'a mut [u8]) -> &'a [u8] {
    if arr.len() == 0 || arr.len() == 1 {
        return arr;
    }
    let mut zero_pos = 0;
    let mut one_pos = 0;
    let mut two_pos = arr.len() - 1;
    //[2, 1, 1, 1, 0, 2, 0, 0] z = 0, o = 0, t=7
    //[0, 1, 1, 1, 0, 2, 0, 2] z = 0, o = 0, t=6
    //[0, 1, 1, 1, 0, 2, 0, 2] z = 1, o = 1, t=6
    //[0, 1, 1, 1, 0, 2, 0, 2] z = 1, o = 1, t=6
    //[0, 1, 1, 1, 0, 2, 0, 2] z = 1, o = 2, t=6
    //[0, 1, 1, 1, 0, 2, 0, 2] z = 1, o = 3, t=6
    //[0, 1, 1, 1, 0, 2, 0, 2] z = 1, o = 4, t=6
    //[0, 0, 1, 1, 1, 2, 0, 2] z = 2, o = 5, t=6
    //[0, 0, 1, 1, 1, 0, 2, 2] z = 2, o = 5, t=5
    //[0, 0, 0, 1, 1, 1, 2, 2] z = 3, o = 6, t=5
    while one_pos <= two_pos {
        if arr[one_pos] == 1 {
            one_pos += 1;
        } else if arr[one_pos] == 0 {
            swap(arr, one_pos, zero_pos);
            one_pos += 1;
            zero_pos += 1;
        } else {
            swap(arr, one_pos, two_pos);
            two_pos -= 1;
        }
    }
    return arr;
}

fn swap(arr: &mut [u8], i: usize, j: usize) {
    let temp = arr[i];
    arr[i] = arr[j];
    arr[j] = temp;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let mut arr = [2, 1, 1, 1, 0, 2, 0, 0];
        let arr = arrange(&mut arr);
        assert_eq!(arr, [0, 0, 0, 1, 1, 1, 2, 2]);
    }

    #[test]
    fn test2() {
        let mut arr = [2, 1, 2, 0, 2, 2, 1, 1, 0, 0, 1, 0, 2, 0, 0];
        let arr = arrange(&mut arr);
        assert_eq!(arr, [0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 2]);
    }
}
