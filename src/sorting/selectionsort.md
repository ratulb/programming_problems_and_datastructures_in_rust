# Selection sort

In selection sort, we start at the beginning of the item's array, look at the rest of array to find the index of any element that could be the smallest the whole array. If found, we we swap the smallest with the element at the first position of the array. Then we look at 
the second element of the array - and try to find its replacement - if any. This process 
continues till we look at all the elements till the element before the last element.

### Below is the insertion sort implementation:
```rust, ignore
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
```
[Source](https://github.com/ratulb/programming_problems_in_rust/blob/master/selectionsort/src/lib.rs)
