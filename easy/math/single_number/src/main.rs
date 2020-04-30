// Given a non-empty array of integers, every element appears twice except for one. Find that single one.

// Note:

// Your algorithm should have a linear runtime complexity. Could you implement it without using extra memory?

// Example 1:

// Input: [2,2,1]
// Output: 1
// Example 2:

// Input: [4,1,2,1,2]
// Output: 4

struct Solution;

// impl Solution {
//     pub fn single_number(nums: Vec<i32>) -> i32 {
//         let mut num = 0;
//         for n in nums.iter() {
//             num ^= *n;
//         }
//         return num;
//     }
// }

use std::collections::HashSet;
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut numbers = HashSet::new();
        let mut sum = 0;
        let mut aim_sum = 0;
        for n in nums.iter() {
            numbers.insert(*n);
            sum += n;
        }

        for n in numbers.iter() {
            aim_sum += n;
        }
        return aim_sum * 2 - sum;
    }
}

fn main() {
    assert_eq!(Solution::single_number(vec![2, 2, 1]), 1);
}
