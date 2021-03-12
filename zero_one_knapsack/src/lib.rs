///0/1-knapsack problem
use std::cmp;
pub fn knapsack(weights: &[u32], values: &[u32], capacity: u32, n: usize) -> u32 {
    if capacity == 0 || n == 0 {
        return 0;
    }
    if weights[n - 1] > capacity {
        return knapsack(weights, values, capacity, n - 1);
    } else {
        return cmp::max(
            values[n - 1] + knapsack(weights, values, capacity - weights[n - 1], n - 1),
            knapsack(weights, values, capacity, n - 1),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::knapsack;
    #[test]
    fn calculate_max_profit() {
        let weights = [10, 20, 30];
        let values = [60, 100, 120];
        let capacity = 50;
        assert_eq!(knapsack(&weights, &values, capacity, weights.len()), 220);
    }
}
