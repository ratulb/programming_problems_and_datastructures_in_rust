///0/1-knapsack problem - with bottom up approach
use std::cmp;

pub fn knapsack(weights: &[u32], values: &[u32], capacity: usize) -> u32 {
    if weights.len() == 0 || values.len() == 0 || capacity < 1 {
        return 0;
    }
    let mut table = vec![vec![0; capacity + 1]; weights.len() + 1];
    //Fill top most row with 0. This signifies 0 value items would fetch no profit
    //This is actually not needed - first we are initially filling up all cells with
    //0 in the above step. This is for illustration purpose
    for j in 0..table[0].len() {
        //capacity + 1
        table[0][j] = 0;
    }
    //Fill up the zeroth column of every row with 0 - to signify the fact that with 0 capacity we
    //will make no profit. Again not required. Only for illustration purpose
    for i in 0..table.len() {
        //weights.len() + 1
        table[i][0] = 0;
    }
    //Now fill up the rest of the cells
    for i in 1..table.len() {
        for j in 1..table[0].len() {
            if weights[i - 1] <= j as u32 {
                //Filling items with weights[i-1] <= capacity(=j)
                //Include the item (hence take it's corresponding value(values[i-1) or exclude item -
                //hence no inclusion of value and therefor no decrease of capacity
                table[i][j] = cmp::max(
                    values[i - 1] + table[i - 1][j - weights[i - 1] as usize],
                    table[i - 1][j],
                );
            } else {
                // weights[i-1] > j
                table[i][j] = table[i - 1][j];
            }
        }
    }
    println!("{:?}", table);
    table[table.len() - 1][table[0].len() - 1] //table[weights.len()][capacity]
}

#[cfg(test)]
mod tests {
    use super::knapsack;
    #[test]
    fn calculate_max_profit1() {
        let weights = [5, 10, 15, 20, 30];
        let values = [30, 40, 60, 100, 120];
        let capacity = 50;
        assert_eq!(knapsack(&weights, &values, capacity), 230);
    }
    #[test]
    fn calculate_max_profit2() {
        let weights = [10, 20, 30, 40];
        let values = [60, 100, 120, 50];
        let capacity = 50;
        assert_eq!(knapsack(&weights, &values, capacity), 220);
    }
}
