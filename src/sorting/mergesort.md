# Merge sort

We present the merge sort merge sort algorithm in this section. Merge sort is is divide and conquer algorithm - We keep dividing the input array (in memory) until each division is no longer than 1 element each. At this point all divinsions are sorted in themselves. Then we call the merge procedure. At this point we need an auxiliary  array - to hold the merged 
halves. From the auxiliary array - we copy the contents back to the original array picking next smaller elements. We don't expect the element type `T` to be clonable - but we need it to have a `Default::default` value - because rust does not like holes in memory.

Following is the implementation:
```rust, ignore
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
```
[Source](https://github.com/ratulb/programming_problems_in_rust/blob/master/mergesort/src/lib.rs)
