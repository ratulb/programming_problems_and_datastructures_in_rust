///Arrange the elements of an array in low/high/low mannner

pub fn array_zigzag(array: &mut [i32]) -> &[i32] {
    if array.len() == 0 {
        return array;
    }
    let mut low = true;
    let mut i = 0;
    while i < array.len() - 1 {
        if low {
            if array[i] > array[i + 1] {
                let temp = array[i];
                array[i] = array[i + 1];
                array[i + 1] = temp;
            }
        } else {
            if array[i] < array[i + 1] {
                let temp = array[i];
                array[i] = array[i + 1];
                array[i + 1] = temp;
            }
        }
        i += 1;
        low = !low;
    }
    array
}

#[cfg(test)]
mod tests {
    use super::array_zigzag;
    #[test]
    fn test_array_zigzag() {
        assert_eq!(
            array_zigzag(&mut [1, 2, 3, 4, 6, 8, 5]),
            &mut [1, 3, 2, 6, 4, 8, 5]
        );
    }
}
