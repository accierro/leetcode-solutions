// Given an array arr, replace every element in that array with the greatest element among the elements to its right, and replace the last element with -1.

// After doing so, return the array.

// Example 1:

// Input: arr = [17,18,5,4,6,1]
// Output: [18,6,6,6,1,-1]

// Constraints:

// 1 <= arr.length <= 10^4
// 1 <= arr[i] <= 10^5

struct Solution;

impl Solution {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![-1; arr.len()];
        let mut max = -1;

        for i in (0..arr.len()).rev() {
            result[i] = max;
            if arr[i] > max {
                max = arr[i];
            }
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::replace_elements(vec![17, 18, 5, 4, 6, 1]),
        vec![18, 6, 6, 6, 1, -1]
    );
}
