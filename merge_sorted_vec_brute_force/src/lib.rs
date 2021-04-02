///Merge a bunch of soprted i32 vecs - brute force

pub fn merge(vecs: &Vec<Vec<i32>>) -> Vec<i32> {
    if vecs.len() == 0 {
        return vec![];
    }
    let mut len = 0;
    for v in vecs {
        len += v.len();
    }
    let mut result = Vec::<i32>::with_capacity(len);
    let mut indices = vec![0; vecs.len()];
    let mut k = 0;
    while k < len {
        let mut min = i32::MAX;
        let mut from_vec = vecs.len(); //Improbable value
        let mut next_value_index = vecs[0].len(); //Improbable value
        for i in 0..vecs.len() {
            let index = indices[i];
            if index >= vecs[i].len() {
                continue;
            }
            if vecs[i][index] < min {
                from_vec = i;
                next_value_index = index;
                min = vecs[from_vec][next_value_index];
            }
            if i == vecs.len() - 1 {
                result.push(min);
                indices[from_vec] = next_value_index + 1;
            }
        }
        k += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::merge;
    #[test]
    fn test_merge_1() {
        let vecs = vec![vec![1, 3, 5, 6, 7], vec![0, 2, 4], vec![2, 4, 6, 8, 9, 10]];
        let result = merge(&vecs);
        assert_eq!(vec![0, 1, 2, 2, 3, 4, 4, 5, 6, 6, 7, 8, 9, 10], result);
    }
}
