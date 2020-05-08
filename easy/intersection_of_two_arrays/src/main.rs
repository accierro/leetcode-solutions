// Given two arrays, write a function to compute their intersection.

// Example 1:

// Input: nums1 = [1,2,2,1], nums2 = [2,2]
// Output: [2]
// Example 2:

// Input: nums1 = [4,9,5], nums2 = [9,4,9,8,4]
// Output: [9,4]
// Note:

// Each element in the result must be unique.
// The result can be in any order.

struct Solution;

use std::collections::HashSet;
use std::iter::FromIterator;
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let first_set: HashSet<i32> = HashSet::from_iter(nums1.into_iter());
        first_set
            .intersection(&HashSet::from_iter(nums2.into_iter()))
            .cloned()
            .collect::<Vec<i32>>()
    }
}

fn main() {
    assert_eq!(
        Solution::intersection(vec![1, 2, 2, 1], vec![2, 2]),
        vec![2]
    );
}
