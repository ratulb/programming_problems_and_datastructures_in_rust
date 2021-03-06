/***
 * Recursively sort a vector of integers
 */

pub fn sort(nums: &mut Vec<i32>) {
    if nums.len() == 0 {
        return;
    } else {
        let last = nums.pop().unwrap();
        sort(nums);
        insert(nums, last);
    }
}

fn insert(nums: &mut Vec<i32>, elem: i32) {
    if nums.len() == 0 || nums[nums.len() - 1] <= elem {
        nums.push(elem);
    } else {
        let last = nums.pop();
        insert(nums, elem);
        nums.push(last.unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn equal_vectors(v1: &Vec<i32>, v2: &Vec<i32>) -> bool {
        v1.len() == v2.len() && v1.iter().zip(v2).all(|(a, b)| *a == *b)
    }
    #[test]
    fn test_sort() {
        let mut v = vec![100, -1, 23, 42, 10, 99, 8, 0];
        sort(&mut v);
        let expected = vec![-1, 0, 8, 10, 23, 42, 99, 100];
        assert_eq!(true, equal_vectors(&v, &expected));
    }
}
