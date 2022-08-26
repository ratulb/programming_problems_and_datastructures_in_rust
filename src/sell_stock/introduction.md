# Sell stock

You are given an array prices where prices[i] is the price of a given stock
on the ith day.

You want to maximize your profit by choosing a single day to buy one stock
and choosing a different day in the future to sell that stock.

Return the maximum profit you can achieve from this transaction. If you cannot
achieve any profit, return 0.

### Implemenation
```rust,ignore
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
```
[Source](https://github.com/ratulb/programming_problems_in_rust/blob/master/buy_and_sell_stock_once/src/lib.rs)
