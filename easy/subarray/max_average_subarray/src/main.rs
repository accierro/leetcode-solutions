// Given an array consisting of n integers, find the contiguous subarray of given length k that has the maximum average value. And you need to output the maximum average value.

// Example 1:

// Input: [1,12,-5,-6,50,3], k = 4
// Output: 12.75
// Explanation: Maximum average is (12-5-6+50)/4 = 51/4 = 12.75

// Note:

// 1 <= k <= n <= 30,000.
// Elements of the given array will be in the range [-10,000, 10,000].

struct Solution;

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut arr = vec![0; nums.len()];
        arr[0] = nums[0];

        for i in 1..nums.len() {
            arr[i] += arr[i - 1] + nums[i];
        }

        let mut result = arr[k as usize - 1];
        for j in k as usize..arr.len() {
            result = std::cmp::max(result, arr[j] - arr[j - k as usize]);
        }

        result as f64 / k as f64
    }
}

fn main() {
    assert_eq!(
        Solution::find_max_average(vec![1, 12, -5, -6, 50, 3], 4),
        12.75
    );
}
