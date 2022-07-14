# Insertion sort

In insertion sort - we presume the item's array to be in two parts - left part which is 
presumed to be sorted and the rest - which is supposed to be unsorted. We pick up the 
unsorted elements one by one and place them in their aprropriate positions in the left 
sorted part untill whole unsorted part becomes empty - when the array is fully sorted.

The sorted part starts with just one element i.e. the first element - which is sorted 
in itself. Proceeding with second element onwards - we keep growing the sorted part. We 
are done when unsorted part becomes exhausted.

### Following is the insertion sort algorithm implementation:
```rust, ignore
///Insertion sort implementation

pub fn sort<T: PartialOrd>(items: &mut [T]) {
    for pos in 1..items.len() {
        let mut i = pos;
        while i > 0 && items[i] < items[i - 1] {
            items.swap(i, i - 1);
            i -= 1;
        }
    }
}
```
[Source](https://github.com/ratulb/programming_problems_in_rust/blob/master/insertionsort/src/lib.rs)
