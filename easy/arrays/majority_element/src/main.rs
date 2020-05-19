// Given an array of size n, find the majority element. The majority element is the element that appears more than ⌊ n/2 ⌋ times.

// You may assume that the array is non-empty and the majority element always exist in the array.

// Example 1:

// Input: [3,2,3]
// Output: 3
// Example 2:

// Input: [2,2,1,1,1,2,2]
// Output: 2

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        let mut result = nums[0];
        let mut max = 1;
        for num in nums.iter() {
            let mut counter = map.entry(*num).or_insert(0);
            *counter += 1;
            if max < *counter {
                result = *num;
                max = *counter;
            }
            if *counter > nums.len() / 2 {
                break;
            }
        }

        result
    }
}

fn main() {
    assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
}
