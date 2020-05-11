// Given an integer array sorted in non-decreasing order, there is exactly one integer in the array that occurs more than 25% of the time.

// Return that integer.

// Example 1:

// Input: arr = [1,2,2,6,6,6,6,7,10]
// Output: 6

// Constraints:

// 1 <= arr.length <= 10^4
// 0 <= arr[i] <= 10^5

struct Solution;

impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let mut max = (1, arr[0]);

        let mut start = 0;
        let mut end = 0;
        let mut prev = arr[0];

        for i in 1..arr.len() {
            end += 1;
            if arr[i] != prev {
                if max.0 < end - start {
                    max.0 = end - start;
                    max.1 = prev;
                }
                prev = arr[i];
                start = i as i32;
                end = i as i32;
            }
        }
        if max.0 < end + 1 - start {
            max.0 = end + 1 - start;
            max.1 = prev;
        }

        max.1
    }
}

fn main() {
    assert_eq!(
        Solution::find_special_integer(vec![1, 2, 2, 6, 6, 6, 6, 7, 10, 10, 10, 10, 10]),
        10
    );
    assert_eq!(Solution::find_special_integer(vec![1, 2, 3, 3]), 3);
}
