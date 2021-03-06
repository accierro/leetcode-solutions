// Given an array of distinct integers arr, find all pairs of elements with the minimum absolute difference of any two elements.

// Return a list of pairs in ascending order(with respect to pairs), each pair [a, b] follows

// a, b are from arr
// a < b
// b - a equals to the minimum absolute difference of any two elements in arr

// Example 1:

// Input: arr = [4,2,1,3]
// Output: [[1,2],[2,3],[3,4]]
// Explanation: The minimum absolute difference is 1. List all pairs with difference equal to 1 in ascending order.
// Example 2:

// Input: arr = [1,3,6,10,15]
// Output: [[1,3]]
// Example 3:

// Input: arr = [3,8,-10,23,19,-4,-14,27]
// Output: [[-14,-10],[19,23],[23,27]]

// Constraints:

// 2 <= arr.length <= 10^5
// -10^6 <= arr[i] <= 10^6

struct Solution;

impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        arr.sort_unstable();
        let mut min = std::i32::MAX;
        for i in 1..arr.len() {
            let first = arr[i - 1];
            let second = arr[i];
            if second - first < min {
                min = second - first;
                result.clear();
                result.push(vec![first, second]);
            } else if second - first == min {
                result.push(vec![first, second]);
            }
        }
        result
    }
}

fn main() {
    assert_eq!(
        Solution::minimum_abs_difference(vec![4, 2, 1, 3]),
        vec![vec![1, 2], vec![2, 3], vec![3, 4]]
    );
}
