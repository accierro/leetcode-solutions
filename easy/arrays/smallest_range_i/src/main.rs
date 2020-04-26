// Given an array A of integers, for each integer A[i] we may choose any x with -K <= x <= K, and add x to A[i].

// After this process, we have some array B.

// Return the smallest possible difference between the maximum value of B and the minimum value of B.

// Example 1:

// Input: A = [1], K = 0
// Output: 0
// Explanation: B = [1]
// Example 2:

// Input: A = [0,10], K = 2
// Output: 6
// Explanation: B = [2,8]
// Example 3:

// Input: A = [1,3,6], K = 3
// Output: 0
// Explanation: B = [3,3,3] or B = [4,4,4]

// Note:

// 1 <= A.length <= 10000
// 0 <= A[i] <= 10000
// 0 <= K <= 10000

struct Solution;

impl Solution {
    pub fn smallest_range_i(a: Vec<i32>, k: i32) -> i32 {
        let mut max = std::i32::MIN;
        let mut min = std::i32::MAX;

        for i in a.iter() {
            if *i > max {
                max = *i;
            }
            if *i < min {
                min = *i;
            }
        }

        if (max - min) / 2 < k {
            return 0;
        }

        return max - min - 2 * k;
    }
}

fn main() {
    assert_eq!(Solution::smallest_range_i(vec![0, 10], 2), 6);
    assert_eq!(Solution::smallest_range_i(vec![0, 10], 10), 0);
    assert_eq!(Solution::smallest_range_i(vec![0, 10], 5), 0);
    assert_eq!(Solution::smallest_range_i(vec![0, 10], 4), 2);
    assert_eq!(Solution::smallest_range_i(vec![1, 10], 4), 1);
}
