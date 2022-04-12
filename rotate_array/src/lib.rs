/***Write a function that rotates a list by k elements. For example, [1, 2, 3, 4, 5, 6] rotated by two becomes [3, 4, 5, 6, 1, 2]. Try solving this without creating a copy of the list. How many swap or move operations do you need?
***/

pub fn rotate_array<T: Default>(arr: &mut [T], times: usize) {
    if arr.len() == 0 || arr.len() == times {
        return;
    }
    let times = times % arr.len();
    for _ in 0..times {
        let mut temp = std::mem::take(&mut arr[0]);
        for j in 1..arr.len() {
            arr[j - 1] = std::mem::take(&mut arr[j]);
        }
        arr[arr.len() - 1] = std::mem::take(&mut temp);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let mut arr = [1, 2, 3, 4, 5, 6];
        rotate_array(&mut arr, 2);
        assert_eq!(arr, [3, 4, 5, 6, 1, 2]);
    }

    #[test]
    fn test2() {
        let mut arr: [i32; 0] = [];
        rotate_array(&mut arr, 2);
        assert_eq!(arr, []);
    }
    #[test]
    fn test3() {
        let mut arr = [1, 2, 3, 4, 5, 6];
        rotate_array(&mut arr, 6);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }
    #[test]
    fn test4() {
        let mut arr = [1, 2, 3, 4, 5, 6];
        rotate_array(&mut arr, 12);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }
    #[test]
    fn test5() {
        let mut arr = [1, 2, 3, 4, 5, 6];
        rotate_array(&mut arr, 8);
        assert_eq!(arr, [3, 4, 5, 6, 1, 2]);
    }
    #[test]
    fn test6() {
        let mut arr = [1, 2, 3, 4, 5, 6];
        rotate_array(&mut arr, 4);
        assert_eq!(arr, [5, 6, 1, 2, 3, 4]);
    }
}
