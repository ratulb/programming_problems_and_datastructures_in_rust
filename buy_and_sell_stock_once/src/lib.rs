/***
You are given an array prices where prices[i] is the price of a given stock
on the ith day.

You want to maximize your profit by choosing a single day to buy one stock
and choosing a different day in the future to sell that stock.

Return the maximum profit you can achieve from this transaction. If you cannot
achieve any profit, return 0.
***/

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_profit = 0;
    let mut left = 0; //Buy stock on day 'left'
    let mut right = 1; //Sell stock on day 'right'

    while right < prices.len() {
        if prices[left] < prices[right] {
            max_profit = std::cmp::max(max_profit, prices[right] - prices[left]);
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
