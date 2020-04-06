// Given a sorted array and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.

// You may assume no duplicates in the array.

// Example 1:

// Input: [1,3,5,6], 5
// Output: 2
// Example 2:

// Input: [1,3,5,6], 2
// Output: 1
// Example 3:

// Input: [1,3,5,6], 7
// Output: 4
// Example 4:

// Input: [1,3,5,6], 0
// Output: 0

struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let index = match nums.iter().position(|&n| n >= target) {
            Some(value) => value,
            None => nums.len(),
        };
        return index as i32;
    }
}

fn main() {
    let vector = vec![1, 2, 3, 4];
    let vector2 = vec![1, 2, 3, 4];
    let vector3 = vec![1, 2, 3, 4];
    assert_eq!(Solution::search_insert(vector, 1), 0);
    assert_eq!(Solution::search_insert(vector2, 5), 4);
    assert_eq!(Solution::search_insert(vector3, 0), 0);
}
