// Given an array A of positive lengths, return the largest perimeter of a triangle with non-zero area, formed from 3 of these lengths.

// If it is impossible to form any triangle of non-zero area, return 0.

// Example 1:

// Input: [2,1,2]
// Output: 5
// Example 2:

// Input: [1,2,1]
// Output: 0
// Example 3:

// Input: [3,2,3,4]
// Output: 10
// Example 4:

// Input: [3,6,2,3]
// Output: 8

// Note:

// 3 <= A.length <= 10000
// 1 <= A[i] <= 10^6

struct Solution;

impl Solution {
    pub fn largest_perimeter(mut a: Vec<i32>) -> i32 {
        a.sort();
        for n in (0..a.len() - 2).rev() {
            let ab = a[n] + a[n + 1];

            if ab > a[n + 2] {
                return ab + a[n + 2];
            }
        }

        0
    }
}

fn main() {
    assert_eq!(Solution::largest_perimeter(vec![2, 1, 2]), 5);
    assert_eq!(Solution::largest_perimeter(vec![1, 2, 1]), 0);
    assert_eq!(Solution::largest_perimeter(vec![3, 2, 3, 4]), 10);
}
