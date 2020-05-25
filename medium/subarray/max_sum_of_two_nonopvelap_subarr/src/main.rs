// Given an array A of non-negative integers, return the maximum sum of elements in two non-overlapping (contiguous) subarrays, which have lengths L and M.  (For clarification, the L-length subarray could occur before or after the M-length subarray.)

// Formally, return the largest V for which V = (A[i] + A[i+1] + ... + A[i+L-1]) + (A[j] + A[j+1] + ... + A[j+M-1]) and either:

// 0 <= i < i + L - 1 < j < j + M - 1 < A.length, or
// 0 <= j < j + M - 1 < i < i + L - 1 < A.length.

// Example 1:

// Input: A = [0,6,5,2,2,5,1,9,4], L = 1, M = 2
// Output: 20
// Explanation: One choice of subarrays is [9] with length 1, and [6,5] with length 2.
// Example 2:

// Input: A = [3,8,1,3,2,1,8,9,0], L = 3, M = 2
// Output: 29
// Explanation: One choice of subarrays is [3,8,1] with length 3, and [8,9] with length 2.
// Example 3:

// Input: A = [2,1,5,6,0,9,5,0,3,8], L = 4, M = 3
// Output: 31
// Explanation: One choice of subarrays is [5,6,0,9] with length 4, and [3,8] with length 3.

// Note:

// L >= 1
// M >= 1
// L + M <= A.length <= 1000
// 0 <= A[i] <= 1000

struct Solution;

impl Solution {
    pub fn max_sum_two_no_overlap(a: Vec<i32>, l: usize, m: usize) -> i32 {
        let mut sums = vec![0; a.len()];
        sums[0] = a[0];
        for n in 1..a.len() {
            sums[n] = sums[n - 1] + a[n];
        }
        let mut res = sums[(l + m - 1) as usize];
        let mut lmax = sums[(l - 1) as usize];
        let mut mmax = sums[(m - 1) as usize];
        for i in (l + m)..a.len() {
            lmax = std::cmp::max(lmax, sums[i - m] - sums[i - l - m]);
            mmax = std::cmp::max(mmax, sums[i - l] - sums[i - l - m]);
            res = std::cmp::max(
                res,
                std::cmp::max(lmax + sums[i] - sums[i - m], mmax + sums[i] - sums[i - l]),
            );
        }

        res
    }
}

fn main() {
    assert_eq!(
        Solution::max_sum_two_no_overlap(vec![0, 6, 5, 2, 2, 5, 1, 9, 4], 1, 2),
        20
    );
}
