// Given an array of integers arr, a lucky integer is an integer which has a frequency in the array equal to its value.

// Return a lucky integer in the array. If there are multiple lucky integers return the largest of them. If there is no lucky integer return -1.

// Example 1:

// Input: arr = [2,2,3,4]
// Output: 2
// Explanation: The only lucky number in the array is 2 because frequency[2] == 2.
// Example 2:

// Input: arr = [1,2,2,3,3,3]
// Output: 3
// Explanation: 1, 2 and 3 are all lucky numbers, return the largest of them.
// Example 3:

// Input: arr = [2,2,2,3,3]
// Output: -1
// Explanation: There are no lucky numbers in the array.
// Example 4:

// Input: arr = [5]
// Output: -1
// Example 5:

// Input: arr = [7,7,7,7,7,7,7]
// Output: 7

// Constraints:

// 1 <= arr.length <= 500
// 1 <= arr[i] <= 500

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut map = HashMap::new();

        for num in arr.iter() {
            map.entry(num).and_modify(|e| *e += 1).or_insert(1);
        }

        let mut max_freq = 0;
        let mut result = -1;
        for (key, val) in map.iter() {
            if *key == val && *val > max_freq && **key > result {
                result = **key;
            }
        }

        result
    }
}

fn main() {
    assert_eq!(Solution::find_lucky(vec![2, 2, 3, 4]), 2);
    assert_eq!(Solution::find_lucky(vec![1, 2, 2, 3, 3, 3]), 3);
    assert_eq!(Solution::find_lucky(vec![2, 2, 2, 3, 3]), -1);
}
