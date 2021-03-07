/***
 * Generate Pascal's triangle
 */

pub fn generate(layers: usize) -> Vec<Vec<u32>> {
    if layers == 0 {
        return vec![];
    }
    let mut triangle = Vec::with_capacity(layers);
    triangle.push(vec![]);
    triangle[0].push(1);
    for i in 1..layers {
        let prev_layer = &triangle[i - 1];
        let mut curr_layer: Vec<u32> = vec![1];
        for j in 1..i {
            curr_layer.push(prev_layer[j - 1] + prev_layer[j]);
        }
        curr_layer.push(1);
        triangle.push(curr_layer);
    }
    triangle
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_no_layer() {
        assert_eq!(generate(0), Vec::<Vec<u32>>::new());
    }
    #[test]
    fn test_one_layer() {
        let layer1 = vec![1];
        let triangle = vec![layer1];
        assert_eq!(generate(1), triangle);
    }
    #[test]
    fn test_multi_layer() {
        let layer1 = vec![1];
        let layer2 = vec![1, 1];
        let layer3 = vec![1, 2, 1];
        let layer4 = vec![1, 3, 3, 1];
        let layer5 = vec![1, 4, 6, 4, 1];
        let triangle = vec![layer1, layer2, layer3, layer4, layer5];
        assert_eq!(generate(5), triangle);
    }
}
