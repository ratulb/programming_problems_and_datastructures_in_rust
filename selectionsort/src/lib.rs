///Selection sort implementation

pub fn sort<T: PartialOrd>(items: &mut [T]) {
    for curr_item_index in 0..items.len() - 1 {
        let mut min_index_in_rest = curr_item_index;
        for next in (curr_item_index + 1)..items.len() {
            if items[next] < items[min_index_in_rest] {
                min_index_in_rest = next;
            }
        }
        if min_index_in_rest != curr_item_index {
            items.swap(curr_item_index, min_index_in_rest);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::sort;
    #[test]
    fn selection_sort_test() {
        let mut items = [10, 2, 100, 56, 9, 11, 200, 11];
        sort(&mut items);
        assert!(items == [2, 9, 10, 11, 11, 56, 100, 200]);
    }
}
