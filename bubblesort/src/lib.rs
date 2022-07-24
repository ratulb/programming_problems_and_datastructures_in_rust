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
                // i += 1;
            }
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::sort;
    #[test]
    fn bubblesort_test() {
        let mut array = [10, 2, 100, 56, 9, 11];
        sort(&mut array);
        assert!(array == [2, 9, 10, 11, 56, 100]);
        let mut array = [10.01, 2.02, 100.001, 56.56, 9.0, 11.11];
        sort(&mut array);
        assert!(array == [2.02, 9.0, 10.01, 11.11, 56.56, 100.001]);
    }
}
