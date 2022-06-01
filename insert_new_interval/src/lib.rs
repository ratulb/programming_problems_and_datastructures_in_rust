/***
 * LeetCode: 57. Insert Interval
 * You are given an array of non-overlapping intervals intervals where intervals[i] = [starti, endi] represent the start and the end of the ith interval and intervals is sorted in ascending order by starti. You are also given an interval newInterval = [start, end] that represents the start and end of another interval.

Insert newInterval into intervals such that intervals is still sorted in ascending order by starti and intervals still does not have any overlapping intervals (merge overlapping intervals if necessary).

Return intervals after the insertion.

Input: intervals = [[1,3],[6,9]], newInterval = [2,5]
Output: [[1,5],[6,9]]

Input: intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]], newInterval = [4,8]
Output: [[1,2],[3,10],[12,16]]
Explanation: Because the new interval [4,8] overlaps with [3,5],[6,7],[8,10].

***/

use std::cmp::max;
use std::cmp::min;

pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    if intervals.len() == 0 || new_interval.len() == 0 {
        return vec![];
    }
    let mut result = Vec::with_capacity(intervals.len() + 1);
    let mut new_interval = new_interval;
    for i in 0..intervals.len() {
        if intervals[i][1] < new_interval[0] {
            result.push(intervals[i].clone());
        } else if new_interval[1] < intervals[i][0] {
            result.push(new_interval);
            result.extend(intervals[i..].to_vec());
            return result;
        } else {
            new_interval = vec![
                min(new_interval[0], intervals[i][0]),
                max(new_interval[1], intervals[i][1]),
            ];
        }
    }
    result.push(new_interval);
    result
}

#[cfg(test)]
mod tests {
    use super::insert;
    #[test]
    fn interval_insert_test_1() {
        let intervals = vec![vec![1, 3], vec![6, 9]];
        let new_interval = vec![2, 5];
        let result = insert(intervals, new_interval);
        assert_eq!(result, vec![vec![1, 5], vec![6, 9]]);
    }

    #[test]
    fn interval_insert_test_2() {
        let intervals = vec![
            vec![1, 2],
            vec![3, 5],
            vec![6, 7],
            vec![8, 10],
            vec![12, 16],
        ];
        let new_interval = vec![4, 8];
        let result = insert(intervals, new_interval);
        assert_eq!(result, vec![vec![1, 2], vec![3, 10], vec![12, 16]]);
    }
}
