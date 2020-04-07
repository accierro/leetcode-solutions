// Given an integer array nums, find the contiguous subarray (containing at least one number) which has the largest sum and return its sum.

// Example:

// Input: [-2,1,-3,4,-1,2,1,-5,4],
// Output: 6
// Explanation: [4,-1,2,1] has the largest sum = 6.
// Follow up:

// If you have figured out the O(n) solution, try coding another solution using the divide and conquer approach, which is more subtle.

struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let length = nums.len();
        let mut dp = vec![0; length];
        dp[0] = nums[0];
        let mut max: i32 = nums[0];

        for i in 1..length {
            dp[i] = nums[i] + (if dp[i - 1] > 0 { dp[i - 1] } else { 0 });
            max = std::cmp::max(max, dp[i]);
        }
        max
    }
}

fn main() {
    assert_eq!(
        Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
        6
    );
}
