struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut result = 0;

        for n in 1..prices.len() {
            let diff = prices[n] - prices[n - 1];
            if diff > 0 {
                result += diff;
            }
        }

        result
    }
}

fn main() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
}
