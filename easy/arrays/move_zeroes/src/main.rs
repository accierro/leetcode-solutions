// Given an array nums, write a function to move all 0's to the end of it while maintaining the relative order of the non-zero elements.

// Example:

// Input: [0,1,0,3,12]
// Output: [1,3,12,0,0]
// Note:

// You must do this in-place without making a copy of the array.
// Minimize the total number of operations.

struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        if nums.len() == 0 {
            return;
        }

        let mut target_idx = 0;
        let mut current_idx = 0;

        while current_idx < nums.len() - 1 {
            current_idx += 1;
            if nums[current_idx] != 0 && nums[target_idx] == 0 {
                let temp = nums[target_idx];
                nums[target_idx] = nums[current_idx];
                nums[current_idx] = temp;
            }
            while target_idx < current_idx && nums[target_idx] != 0 {
                target_idx += 1;
            }
        }
    }
}

fn main() {
    let mut arr = vec![0, 1, 0, 3, 12];
    Solution::move_zeroes(&mut arr);
    assert_eq!(arr, vec![1, 3, 12, 0, 0]);
}
