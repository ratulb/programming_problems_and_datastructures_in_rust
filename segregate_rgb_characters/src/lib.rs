/***
 * Given an array of characters 'R', 'G' & 'B' - sort them such that 'R's are followed by
 * 'G' and then 'B'
 ***/

pub fn sort_characters(arr: &mut [char]) {
    if arr.len() == 0 || arr.len() == 1 {
        return;
    }
    let mut red = 0;
    let mut green = 0;
    let mut blue = arr.len() - 1;

    while green <= blue {
        if arr[green] == 'G' {
            green += 1;
        } else if arr[green] == 'R' {
            arr.swap(green, red);
            green += 1;
            red += 1;
        } else {
            arr.swap(green, blue);
            blue -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sort_characters_test_1() {
        let mut arr = ['G', 'B', 'R', 'R', 'B', 'R', 'G'];
        sort_characters(&mut arr);
        assert_eq!(arr, ['R', 'R', 'R', 'G', 'G', 'B', 'B']);
    }
}
