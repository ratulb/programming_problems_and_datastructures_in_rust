///Merge sort implementation
pub fn sort<'a, T: PartialOrd + Default>(items: &'a mut [T]) -> &'a [T] {
    if items.len() == 0 {
        return items;
    }
    let mut aux = Vec::with_capacity(items.len());
    //Put default value - since don't expect `T` to be clone - we
    //can not use Vec `fill`
    for _ in 0..items.len() {
        aux.push(T::default());
    }
    mergesort(items, 0, items.len() - 1, &mut aux);
    items
}
pub fn mergesort<T: PartialOrd + Default>(
    items: &mut [T],
    left: usize,
    right: usize,
    aux: &mut Vec<T>,
) {
    if right - left < 1 {
        return;
    }
    let mid = left + (right - left) / 2;
    mergesort(items, left, mid, aux);
    mergesort(items, mid + 1, right, aux);
    if items[mid] <= items[mid + 1] {
        return;
    }
    merge(items, left, mid, right, aux);
}
pub fn merge<T: PartialOrd + Default>(
    items: &mut [T],
    left: usize,
    mid: usize,
    right: usize,
    aux: &mut Vec<T>,
) {
    //Copy elements to the auxiliary array
    for i in left..=right {
        aux.insert(i, std::mem::take(&mut items[i]));
    }
    //Start of left half
    let mut left_index = left;
    //Start of right half
    let mut right_index = mid + 1;
    //From left to right all the way
    for item_index in left..=right {
        //Left half got exhausted
        //Copy what we have in the auxilary array to the items array
        if left_index > mid {
            items[item_index] = std::mem::take(&mut aux[right_index]);
            right_index += 1;
        //Right half exhausted
        } else if right_index > right {
            items[item_index] = std::mem::take(&mut aux[left_index]);
            left_index += 1;
        } else if aux[left_index] < aux[right_index] {
            items[item_index] = std::mem::take(&mut aux[left_index]);
            left_index += 1;
        } else {
            items[item_index] = std::mem::take(&mut aux[right_index]);
            right_index += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::sort;
    use rand::Rng;
    #[test]
    fn merge_sort_test() {
        let mut items = [89, 23, 11, 11, 11, 23, 42];
        sort(&mut items);
        assert!(is_sorted(&items));
        let mut items = [89, 23];
        sort(&mut items);
        assert!(is_sorted(&items));
        let mut items = [89];
        sort(&mut items);
        assert!(is_sorted(&items));
        let items: &mut [i32] = &mut [];
        sort(items);
        assert!(is_sorted(&items));
        let mut runs = 50;
        loop {
            let mut items: [u8; 10] = [0; 10];
            rand::thread_rng().fill(&mut items);
            sort(&mut items);
            if !is_sorted(&items) {
                panic!("Array is not sorted...");
            }
            runs -= 1;
            if runs == 0 {
                break;
            }
        }
    }
    fn is_sorted<T: PartialOrd>(items: &[T]) -> bool {
        for idx in 1..items.len() {
            if items[idx - 1] > items[idx] {
                return false;
            }
        }
        true
    }
}
