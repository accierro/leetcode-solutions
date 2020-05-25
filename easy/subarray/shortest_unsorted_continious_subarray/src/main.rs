// Given an integer array, you need to find one continuous subarray that if you only sort this subarray in ascending order, then the whole array will be sorted in ascending order, too.

// You need to find the shortest such subarray and output its length.

// Example 1:
// Input: [2, 6, 4, 8, 10, 9, 15]
// Output: 5
// Explanation: You need to sort [6, 4, 8, 10, 9] in ascending order to make the whole array sorted in ascending order.
// Note:
// Then length of the input array is in range [1, 10,000].
// The input array may contain duplicates, so ascending order here means <=.

struct Solution;

impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let mut lo = 0;
        let mut hi = nums.len() - 1;

        while lo < hi {
            if nums[lo] > nums[lo + 1] && nums[hi] < nums[hi - 1] {
                break;
            }
            if nums[lo] <= nums[lo + 1] {
                lo += 1;
            }
            if nums[hi] >= nums[hi - 1] {
                hi -= 1;
            }
        }
        if hi <= lo {
            0
        } else {
            (hi - lo + 1) as i32
        }
    }
}

fn main() {
    assert_eq!(Solution::find_unsorted_subarray(vec![0, 3, 2, 2, 2]), 4);
    assert_eq!(Solution::find_unsorted_subarray(vec![0, 1, 1, 1, 1]), 0);
    assert_eq!(
        Solution::find_unsorted_subarray(vec![2, 6, 4, 8, 10, 9, 15]),
        5
    );
    assert_eq!(
        Solution::find_unsorted_subarray(vec![2, 6, 4, 8, 10, 9, 9, 15]),
        5
    );
}
