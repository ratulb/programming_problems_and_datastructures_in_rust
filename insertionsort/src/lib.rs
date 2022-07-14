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
#[cfg(test)]
mod tests {
    use super::sort;
    #[test]
    fn sort_test() {
        let mut items = [10, 2, 100, 56, 9, 11, 11];
        sort(&mut items);
        assert!(items == [2, 9, 10, 11, 11, 56, 100]);
    }
}
