///0/1-knapsack problem - with memoization
use std::cmp;

pub fn knapsack(weights: &[u32], values: &[u32], capacity: usize) -> u32 {
    knapsack_helper(weights, values, capacity, weights.len(), &mut None)
}

pub fn knapsack_helper(
    weights: &[u32],
    values: &[u32],
    capacity: usize,
    n: usize,
    mem_table: &mut Option<Vec<Vec<u32>>>,
) -> u32 {
    match mem_table {
        None => {
            *mem_table = Some(vec![vec![0; capacity + 1]; n + 1]);
        }
        Some(table) => {
            if table[n][capacity] != 0 {
                let cached_computation = table[n][capacity];
                println!("Cache hit: {}", cached_computation);
                return cached_computation;
            }
        }
    };

    if capacity == 0 || n == 0 {
        return 0;
    }

    if weights[n - 1] > capacity as u32 {
        let value = knapsack_helper(weights, values, capacity, n - 1, mem_table);
        if let Some(ref mut v) = mem_table {
            v[n][capacity] = value;
        }
        value
    } else {
        let value = cmp::max(
            values[n - 1]
                + knapsack_helper(
                    weights,
                    values,
                    capacity - weights[n - 1] as usize,
                    n - 1,
                    mem_table,
                ),
            knapsack_helper(weights, values, capacity, n - 1, mem_table),
        );
        if let Some(ref mut v) = mem_table {
            v[n][capacity] = value;
        }
        value
    }
}

#[cfg(test)]
mod tests {
    use super::knapsack;
    #[test]
    fn calculate_max_profit() {
        let weights = [5, 10, 15, 20, 30];
        let values = [30, 40, 60, 100, 120];
        let capacity = 50;
        assert_eq!(knapsack(&weights, &values, capacity), 230);
    }
}
