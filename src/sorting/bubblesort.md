# Bubble sort

Bubble sort algorithm starts with left most item in the given array and compares it to the element next to it and swaps them if next is smaller than the previous. Then it starts with the next item( irrespective of whether it was swapped or not in the previous step) and 
compares with it's next and the process is continued. In the first pass, the comparison and swapping (if necessary) goes all the way till the end of the array and the biggest element in the array lands in the right most position. Again second pass starts with the left most element, puts next bigger element in the array in one position ahead of the biggest 
element. We maintain a `boolean` flag to avoid further unnecessary iterations if no 
swapping was done in the previous pass (because array is already sorted).

### Following is the bubble sort implementation:
```rust, ignore
///Bubble sort implementation

pub fn sort<T: PartialOrd>(items: &mut [T]) {
    let mut swapped = true;
    while swapped {
        swapped = false;
        let mut i = 0;
        for j in 0..items.len() - 1 - i {
            if items[j] > items[j + 1] {
                items.swap(j, j + 1);
                swapped = true;
                i += 1;
            }
        }
    }
}
```
[Source](https://github.com/ratulb/programming_problems_in_rust/tree/master/bubblesort/src/lib.rs)
