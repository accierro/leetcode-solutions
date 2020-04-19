// Given an array of integers A sorted in non-decreasing order, return an array of the squares of each number, also in sorted non-decreasing order.

// Example 1:

// Input: [-4,-1,0,3,10]
// Output: [0,1,9,16,100]
// Example 2:

// Input: [-7,-3,2,3,11]
// Output: [4,9,9,49,121]

// Note:

// 1 <= A.length <= 10000
// -10000 <= A[i] <= 10000
// A is sorted in non-decreasing order.

struct Solution;

impl Solution {
    pub fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; a.len()];

        let mut i = 0;
        let mut j = a.len() - 1;

        for p in (0..result.len()).rev() {
            if a[i].abs() > a[j].abs() {
                result[p] = a[i] * a[i];
                i += 1;
            } else {
                result[p] = a[j] * a[j];
                j -= 1;
            }
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::sorted_squares(vec![-4, -1, 0, 3, 10]),
        vec![0, 1, 9, 16, 100]
    )
}
