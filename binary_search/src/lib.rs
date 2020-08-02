pub fn binary_search<T>(array: &[T], elem: T) -> Option<usize>
where
    T: PartialEq + PartialOrd,
{
    let mut low: i32 = 0;
    let mut high: i32 = (array.len() - 1) as i32;
    while low <= high {
        let mid_idx = low + (high - low) / 2;
        let mid = mid_idx as usize;

        if array[mid] == elem {
            return Some(mid);
        } else if array[mid] < elem {
            low = mid as i32 + 1;
        } else {
            high = mid as i32 - 1;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::binary_search;
    use rand::Rng;
    #[test]
    fn binary_search_test() {
        let mut array: [u8; 10] = [0; 10];
        rand::thread_rng().fill(&mut array);
        array.sort();
        let elem = array[0];
        let index = binary_search(&array, elem);
        assert_eq!(Some(0), index);
    }
}
