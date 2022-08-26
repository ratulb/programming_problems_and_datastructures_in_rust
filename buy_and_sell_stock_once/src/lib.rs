//Buy and sell stock once - max profit
use std::cmp::max;
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_profit = 0;
    let mut left = 0; //Buy stock on day 'left'
    let mut right = 1; //Sell stock on day 'right'

    while right < prices.len() {
        if prices[left] < prices[right] {
            max_profit = max(max_profit, prices[right] - prices[left]);
        } else {
            left = right;
        }
        right += 1;
    }
    max_profit
}

#[cfg(test)]
mod tests {
    use super::max_profit;
    #[test]
    fn test_max_profit_1() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        let result = max_profit(prices);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_max_profit_2() {
        let prices = vec![7, 6, 4, 3, 1];
        let result = max_profit(prices);
        assert_eq!(result, 0);
    }
}
