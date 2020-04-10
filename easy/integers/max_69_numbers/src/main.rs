// Given a positive integer num consisting only of digits 6 and 9.

// Return the maximum number you can get by changing at most one digit (6 becomes 9, and 9 becomes 6).

// Example 1:

// Input: num = 9669
// Output: 9969
// Explanation:
// Changing the first digit results in 6669.
// Changing the second digit results in 9969.
// Changing the third digit results in 9699.
// Changing the fourth digit results in 9666.
// The maximum number is 9969.
// Example 2:

// Input: num = 9996
// Output: 9999
// Explanation: Changing the last digit 6 to 9 results in the maximum number.
// Example 3:

// Input: num = 9999
// Output: 9999
// Explanation: It is better not to apply any change.

// Constraints:

// 1 <= num <= 10^4
// num's digits are 6 or 9.
use std::convert::TryInto;
struct Solution;

impl Solution {
    pub fn maximum69_number(num: i32) -> i32 {
        let mut n = num;
        let mut i = 0;
        let mut index: i32 = -1;
        while n != 0 {
            let digit = n % 10;
            if digit == 6 {
                index = i;
            }
            i += 1;
            n /= 10;
        }

        if index == -1 {
            return num;
        }
        num + 3 * 10_i32.pow((index as u32).try_into().unwrap())
    }
}

fn main() {
    assert_eq!(Solution::maximum69_number(9669), 9969);
    assert_eq!(Solution::maximum69_number(9996), 9999);
    assert_eq!(Solution::maximum69_number(669), 969);
}
