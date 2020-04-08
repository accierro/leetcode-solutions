// Given the array nums, for each nums[i] find out how many numbers in the array are smaller than it. That is, for each nums[i] you have to count the number of valid j's such that j != i and nums[j] < nums[i].

// Return the answer in an array.

// Example 1:

// Input: nums = [8,1,2,2,3]
// Output: [4,0,1,1,3]
// Explanation:
// For nums[0]=8 there exist four smaller numbers than it (1, 2, 2 and 3).
// For nums[1]=1 does not exist any smaller number than it.
// For nums[2]=2 there exist one smaller number than it (1).
// For nums[3]=2 there exist one smaller number than it (1).
// For nums[4]=3 there exist three smaller numbers than it (1, 2 and 2).
// Example 2:

// Input: nums = [6,5,4,8]
// Output: [2,1,0,3]
// Example 3:

// Input: nums = [7,7,7,7]
// Output: [0,0,0,0]

// Constraints:

// 2 <= nums.length <= 500
// 0 <= nums[i] <= 100

struct Solution;

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut bucket: [i32; 101] = [0; 101];
        let mut result: Vec<i32> = Vec::new();
        for num in nums.iter() {
            bucket[*num as usize] += 1;
        }

        for i in 1..bucket.len() {
            bucket[i] += bucket[i - 1];
        }
        for i in 0..bucket.len() {}

        for i in 0..nums.len() {
            if nums[i] == 0 {
                result.push(0);
            } else {
                result.push(bucket[(nums[i] - 1) as usize]);
            }
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::smaller_numbers_than_current(vec![8, 1, 2, 2, 3]),
        vec![4, 0, 1, 1, 3]
    );
}
