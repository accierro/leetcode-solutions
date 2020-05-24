// An array is monotonic if it is either monotone increasing or monotone decreasing.

// An array A is monotone increasing if for all i <= j, A[i] <= A[j].  An array A is monotone decreasing if for all i <= j, A[i] >= A[j].

// Return true if and only if the given array A is monotonic.

// Example 1:

// Input: [1,2,2,3]
// Output: true
// Example 2:

// Input: [6,5,4,4]
// Output: true
// Example 3:

// Input: [1,3,2]
// Output: false
// Example 4:

// Input: [1,2,4,5]
// Output: true
// Example 5:

// Input: [1,1,1]
// Output: true

// Note:

// 1 <= A.length <= 50000
// -100000 <= A[i] <= 100000

struct Solution;

use std::cmp::Ordering;
impl Solution {
    pub fn is_monotonic(a: Vec<i32>) -> bool {
        if a.len() == 1 {
            return true;
        }
        let mut ord = a[1].cmp(&a[0]);
        for n in 1..a.len() {
            let cmp_res = a[n].cmp(&a[n - 1]);
            if ord == Ordering::Equal {
                ord = cmp_res;
            }
            if cmp_res != ord && cmp_res != Ordering::Equal {
                return false;
            }
        }

        true
    }
}

fn main() {
    assert_eq!(Solution::is_monotonic(vec![1, 2, 2, 3]), true);
    assert_eq!(Solution::is_monotonic(vec![1, 2, 0, 3]), false);
    assert_eq!(Solution::is_monotonic(vec![3, 2, 2, 1]), true);
    assert_eq!(Solution::is_monotonic(vec![3, 2, 6, 1]), false);
    assert_eq!(Solution::is_monotonic(vec![1, 1, 0]), true);
}
