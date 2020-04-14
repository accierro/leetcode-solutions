// In a array A of size 2N, there are N+1 unique elements, and exactly one of these elements is repeated N times.

// Return the element repeated N times.

// Example 1:

// Input: [1,2,3,3]
// Output: 3
// Example 2:

// Input: [2,1,2,5,3,2]
// Output: 2
// Example 3:

// Input: [5,1,5,2,5,3,5,4]
// Output: 5

// Note:

// 4 <= A.length <= 10000
// 0 <= A[i] < 10000
// A.length is even

use std::collections::HashSet;
struct Solution;

impl Solution {
    pub fn repeated_n_times(a: Vec<i32>) -> i32 {
        let mut set = HashSet::new();

        for num in a.iter() {
            if !set.insert(num) {
                return *num;
            }
        }
        0
    }
}

fn main() {
    assert_eq!(Solution::repeated_n_times(vec![1, 2, 3, 3]), 3);
}
