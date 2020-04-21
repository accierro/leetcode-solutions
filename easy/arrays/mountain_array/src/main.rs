// Let's call an array A a mountain if the following properties hold:

// A.length >= 3
// There exists some 0 < i < A.length - 1 such that A[0] < A[1] < ... A[i-1] < A[i] > A[i+1] > ... > A[A.length - 1]
// Given an array that is definitely a mountain, return any i such that A[0] < A[1] < ... A[i-1] < A[i] > A[i+1] > ... > A[A.length - 1].

// Example 1:

// Input: [0,1,0]
// Output: 1
// Example 2:

// Input: [0,2,1,0]
// Output: 1
// Note:

// 3 <= A.length <= 10000
// 0 <= A[i] <= 10^6
// A is a mountain, as defined above.

struct Solution;

impl Solution {
    pub fn peak_index_in_mountain_array(a: Vec<i32>) -> i32 {
        let mut mid = (a.len() - 1) / 2;
        let mut lowest = 0;
        let mut highest = a.len() - 1;
        while mid > lowest && mid < highest {
            if a[mid] > a[mid - 1] && a[mid] > a[mid + 1] {
                return mid as i32;
            }
            if a[mid] < a[mid - 1] && a[mid] > a[mid + 1] {
                highest = mid;
                mid = (lowest + highest) / 2;
            }
            if a[mid] > a[mid - 1] && a[mid] < a[mid + 1] {
                lowest = mid;
                mid = (lowest + highest) / 2;
            }
        }
        mid as i32
    }
}

fn main() {
    assert_eq!(Solution::peak_index_in_mountain_array(vec![3, 4, 5, 1]), 2);
    assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 1, 0]), 1);
    assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 2, 1, 0]), 1);
}
