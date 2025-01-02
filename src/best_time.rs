// You are given an array prices where prices[i] is the price of a given stock on the ith day.

// You want to maximize your profit by choosing a single day to buy one stock and choosing a different day 
// in the future to sell that stock.

// Return the maximum profit you can achieve from this transaction. If you cannot achieve any profit, return 0.

use std::collections::HashMap;

pub enum BestTimeError {
    NoDays,
}
// [14, 8, 3, 4, 4, 6, 14, 2, 13, 10];
pub fn max_profit(prices: &Vec<i32>) -> Result<(usize, usize, i32), BestTimeError> {

    if prices.len() == 0 {
        return Err(BestTimeError::NoDays)
    }

    let mut buy_day = 0;
    let mut sell_day = 0;
    let mut max_profit = 0;

    let mut min_price = prices[0];
    let mut min_price_index = 0;

    prices
    .iter()
    .enumerate()
    .for_each(|(current_day, price)| {
        if *price < min_price {
            min_price = *price;
            min_price_index = current_day;
        } else {
            let current_profit = price - min_price;
            if current_profit > max_profit {
                buy_day = min_price_index;
                sell_day = current_day;
                max_profit = current_profit
            }
        }
    });

    Ok((buy_day, sell_day, max_profit))
}