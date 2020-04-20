// Given an array of 2n integers, your task is to group these integers into n pairs of integer, say (a1, b1), (a2, b2), ..., (an, bn) which makes sum of min(ai, bi) for all i from 1 to n as large as possible.

// Example 1:
// Input: [1,4,3,2]

// Output: 4
// Explanation: n is 2, and the maximum sum of pairs is 4 = min(1, 2) + min(3, 4).
// Note:
// n is a positive integer, which is in the range of [1, 10000].
// All the integers in the array will be in the range of [-10000, 10000].

struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::new();
        let mut total_sum = 0;
        for num in nums.iter() {
            heap.push(num);
        }

        for i in 0..nums.len() / 2 {
            total_sum += std::cmp::min(heap.pop().unwrap(), heap.pop().unwrap());
        }

        total_sum
    }

    pub fn array_pair_sum2(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums.iter().step_by(2).sum()
    }
}

fn main() {
    assert_eq!(Solution::array_pair_sum(vec![1, 4, 3, 2]), 4);
}
