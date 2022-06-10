use std::collections::BinaryHeap;
/**
Given a list of points, a central point, and an integer k, find the nearest k
points from the central point.

For example, given the list of points [(0, 0), (5, 4), (3, 1)], the central
point (1, 2), and k = 2, return [(0, 0), (3, 1)]

***/
use std::collections::HashMap;
pub fn k_closest(points: Vec<Vec<i32>>, k: i32, from: (i32, i32)) -> Vec<Vec<i32>> {
    if points.len() == 0 {
        return vec![];
    }
    let k = k as usize;
    if k >= points.len() {
        return points;
    }
    let mut result = vec![];
    let from = Point::new(from.0, from.1);
    let mut multi_map = HashMap::<i32, Vec<Point>>::new();
    for point in points {
        let point = Point::new(point[0], point[1]);
        let distance = point.distance(&from);
        multi_map.entry(distance).or_insert(Vec::new()).push(point);
    }
    let mut heap = BinaryHeap::new();
    let mut keys = multi_map.keys();
    let mut index = 0;
    while let Some(key) = keys.next() {
        if index < k {
            heap.push(key);
            index += 1;
            continue;
        }
        match heap.peek() {
            Some(v) if v > &key => {
                heap.pop();
                heap.push(key);
            }
            _ => continue,
        }
    }
    assert_eq!(heap.len(), k);
    let mut queue = Vec::with_capacity(k);
    while !heap.is_empty() {
        queue.push(heap.pop().unwrap());
    }

    while !queue.is_empty() {
        result.extend(
            multi_map
                .get(queue.pop().unwrap())
                .unwrap()
                .iter()
                .map(|p| vec![p.0, p.1]),
        );
        if result.len() == k {
            break;
        }
    }
    result
}

#[derive(Debug)]
struct Point(i32, i32);
impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point(x, y)
    }
    fn distance(&self, point: &Point) -> i32 {
        (self.0 - point.0).pow(2) + (self.1 - point.1).pow(2)
    }
}

#[cfg(test)]
mod tests {
    use super::k_closest;
    #[test]
    fn test_k_closest() {
        let points = vec![vec![0, 0], vec![5, 4], vec![3, 1]];
        let k = 2;
        let from = (1, 2);
        let result = k_closest(points, k, from);
        assert_eq!(result, vec![vec![0, 0], vec![3, 1]]);

        let points = vec![vec![1, 3], vec![-2, 2]];
        let k = 1;
        let from = (0, 0);
        let result = k_closest(points, k, from);
        assert_eq!(result, vec![vec![-2, 2]]);

        let points = vec![vec![3, 3], vec![5, -1], vec![-2, 4]];
        let k = 2;
        let from = (0, 0);
        let result = k_closest(points, k, from);
        assert_eq!(result, vec![vec![3, 3], vec![-2, 4]]);
    }
}
