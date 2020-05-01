// Given an integer n. Each number from 1 to n is grouped according to the sum of its digits.

// Return how many groups have the largest size.

// Example 1:

// Input: n = 13
// Output: 4
// Explanation: There are 9 groups in total, they are grouped according sum of its digits of numbers from 1 to 13:
// [1,10], [2,11], [3,12], [4,13], [5], [6], [7], [8], [9]. There are 4 groups with largest size.
// Example 2:

// Input: n = 2
// Output: 2
// Explanation: There are 2 groups [1], [2] of size 1.
// Example 3:

// Input: n = 15
// Output: 6
// Example 4:

// Input: n = 24
// Output: 5

struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut bucket = HashMap::new();
        for mut num in 1..n + 1 {
            let mut sum = 0;
            while num > 0 {
                sum += num % 10;
                num /= 10;
            }
            bucket.entry(sum).and_modify(|e| *e += 1).or_insert(1);
        }

        let mut count = 0;
        let mut max = std::i32::MIN;
        for val in bucket.values() {
            if *val > max {
                max = *val;
                count = 1;
            } else if *val == max {
                count += 1;
            }
        }

        count
    }
}

fn main() {
    assert_eq!(Solution::count_largest_group(15), 6);
    assert_eq!(Solution::count_largest_group(2), 2);
    assert_eq!(Solution::count_largest_group(24), 5);
    assert_eq!(Solution::count_largest_group(46), 6);
}
