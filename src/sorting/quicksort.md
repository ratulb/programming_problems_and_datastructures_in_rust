# Quick sort

In this section we present the implementtion of quick sort algorithm. Quick sort is an efficient 
sorting technique that uses no extra storage and can perform at `nlog(n)` time when the array 
elements are randomly placed. 

Here we randomly select left, right or middle element as the pivot while the algorithm is process-  ing sub arrays.

### Following is the implementation:
```rust, ignore

Please ask your administrator.
///Implementation of Quick Sort algorithm
use rand::Rng;

pub fn quicksort<T: PartialOrd>(array: &mut [T]) {
    let mut rng = rand::thread_rng();
    sort(array, 0, array.len() - 1, &mut rng);
}
//Uses the right most element as pivot
#[inline]
fn right_as_pivot<T: PartialOrd>(array: &mut [T], left: usize, right: usize) -> usize {
    let mut i = left;
    for j in left..right {
        if array[j] <= array[right] {
            array.swap(i, j);
            i += 1;
        }
    }
    array.swap(i, right);
    i
}
//Uses left most element as the as pivot
#[inline]
fn left_as_pivot<T: PartialOrd>(array: &mut [T], left: usize, right: usize) -> usize {
    let mut i = right;
    for j in (left + 1..=right).rev() {
        if array[j] > array[left] {
            array.swap(i, j);
            i -= 1;
        }
    }
    array.swap(i, left);
    i
}
//Uses left most element as the as pivot
#[inline]
fn mid_as_pivot<T: PartialOrd>(array: &mut [T], left: usize, right: usize) -> usize {
    let mid = left + (right - left) / 2;
    array.swap(left, mid);
    let mut i = right;
    for j in (left + 1..=right).rev() {
        if array[j] > array[left] {
            array.swap(i, j);
            i -= 1;
        }
    }
    array.swap(i, left);
    i
}

//Recursive parttion sort algorithm

fn sort<T: PartialOrd>(array: &mut [T], left: usize, right: usize, random: &mut impl Rng) {
    if left < right {
        let partition = partition(array, left, right, random);
        if array[..partition].len() > 1 {
            sort(array, left, partition - 1, random);
        }
        if array[partition + 1..].len() > 1 {
            sort(array, partition + 1, right, random);
        }
    }
}

///Parttion the array segments using either left or right most elements as
///the pivots randomly.
#[inline]
fn partition<T: PartialOrd>(
    array: &mut [T],
    left: usize,
    right: usize,
    random: &mut impl Rng,
) -> usize {
    if random.gen::<bool>() {
        left_as_pivot(array, left, right)
    } else if random.gen::<bool>() {
        right_as_pivot(array, left, right)
    } else {
        mid_as_pivot(array, left, right)
    }
}

#[cfg(test)]
mod tests {
    use super::quicksort;
    use rand::Rng;
    #[test]
    fn quicksort_test() {
        let mut runs = 50000;
        loop {
            let mut array: [u16; 20] = [0; 20];
            rand::thread_rng().fill(&mut array);
            quicksort(&mut array);
            if !is_sorted(&array) {
                panic!("Array is not sorted...");
            }
            runs -= 1;
            if runs == 0 {
                break;
            }
        }
    }
    fn is_sorted(array: &[u16]) -> bool {
        for idx in 1..array.len() {
            if array[idx - 1] > array[idx] {
                return false;
            }
        }
        true
    }
}
```

Are you ok? Are you ok? Are you ok? Are you ok?

[Source](https://github.com/ratulb/programming_problems_in_rust/blob/master/quicksort/src/lib.rs)
