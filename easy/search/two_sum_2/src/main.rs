// Given an array of integers that is already sorted in ascending order, find two numbers such that they add up to a specific target number.

// The function twoSum should return indices of the two numbers such that they add up to the target, where index1 must be less than index2.

// Note:

// Your returned answers (both index1 and index2) are not zero-based.
// You may assume that each input would have exactly one solution and you may not use the same element twice.
// Example:

// Input: numbers = [2,7,11,15], target = 9
// Output: [1,2]
// Explanation: The sum of 2 and 7 is 9. Therefore index1 = 1, index2 = 2.

struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i = 0;
        let mut j = numbers.len() - 1;
        let mut s;
        while i < j {
            s = numbers[i] + numbers[j];
            if (s == target) {
                return vec![(i + 1) as i32, (j + 1) as i32];
            } else if (s > target) {
                j -= 1;
            } else {
                i += 1;
            }
        }
        vec![]
    }
}

fn main() {
    assert_eq!(Solution::two_sum(vec![2, 3, 4], 6), vec![1, 3]);
    assert_eq!(
        Solution::two_sum(
            vec![1, 2, 3, 5, 6, 9, 12, 13, 15, 20, 90, 91, 92, 93, 95, 100],
            20
        ),
        vec![4, 9]
    );
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2])
}
