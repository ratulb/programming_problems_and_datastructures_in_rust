use std::collections::BinaryHeap;

pub fn k_nearest(points: &[(i32, i32)], k: usize) -> Vec<(i32, i32)> {
    if points.len() == 0 || k > points.len() {
        return vec![];
    } else {
        let mut min_heap = BinaryHeap::new();
        for i in 0..k {
            let mut p = Point {
                x: points[i].0,
                y: points[i].1,
                d: 0.0,
            };
            p.distance();
            min_heap.push(p);
        }
        for j in k..points.len() {
            let mut next = Point {
                x: points[j].0,
                y: points[j].1,
                d: 0.0,
            };
            next.distance();
            if min_heap.peek().unwrap().d > next.d {
                min_heap.pop();
                min_heap.push(next);
            }
        }
        let mut k_nearest = Vec::<(i32, i32)>::with_capacity(k);
        for _ in 0..k {
            let popped = min_heap.pop().unwrap();
            k_nearest.push((popped.x, popped.y));
        }
        return k_nearest;
    }
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
    d: f32,
}

use std::cmp::Ord;
use std::cmp::Ordering;

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        //self.d.cmp(&other.d)
        if self.d < other.d {
            Ordering::Greater
        } else if self.d > other.d {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cmp(self))
    }
}
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.d == other.d
    }
}

impl Eq for Point {}

impl Point {
    pub fn distance(&mut self) -> f32 {
        let d_sqd = self.x.pow(2) + self.y.pow(2);
        self.d = ((f32::sqrt(d_sqd as f32)) * 1000.0).round() / 1000.0;
        self.d
    }
}
#[cfg(test)]
mod tests {
    use super::k_nearest;
    #[test]
    fn test_k_nearest() {
        let points = [
            (2, 3),
            (10, 1),
            (1, 2),
            (2, 9),
            (4, 1),
            (21, 3),
            (2, 2),
            (-1, 2),
            (3, -1),
        ];
        assert_eq!(
            k_nearest(&points, 5),
            vec![(2, 3), (3, -1), (2, 2), (-1, 2), (1, 2)]
        );
    }
}
